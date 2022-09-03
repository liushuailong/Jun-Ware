use std::intrinsics::atomic_xor;
use crate::layout::{LayoutBox, Rect};
use std::io::{self, Write, Seek, SeekFrom};
use crate::painting::{build_display_list, DisplayCommand};


fn px_to_pt(value: f32) -> f32 {
    value * 0.75
}

pub fn render<W: Write + Seek>(layout_root: &LayoutBox, bounds: Rect, file: &mut W) -> io::Result<()> {
    let display_list = build_display_list(layout_root);
    let mut pdf = Pdf::new(file)?;
    pdf.render_page(px_to_pt(bounds.width), px_to_pt(bounds.height), |output| {
        for item in display_list {
            render_item(&item, output)?;
        }
        Ok(())
    })?;
    pdf.finish()
}

fn render_item<W: Write>(item: &DisplayCommand, output: &mut W) -> io::Result<()> {
    match *item {
        DisplayCommand::SolidColor(color, rect) => {
            writeln!(
                output,
                "{} {} {} sc {} {} {} {} re f",
                color.r,
                color.g,
                color.b,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
            )
        }
    }
}

struct Pdf<'a, W: 'a + Write + Seek> {
    output: &'a mut W,
    object_offsets: Vec<i64>,
    page_objects_ids: Vec<usize>,
}

const ROOT_OBJECT_ID: usize = 1;
const PAGES_OBJECT_ID: usize = 2;

impl <'a, W: Write + Seek> Pdf<'a, W> {
    fn new(output: &'a mut W) -> io::Result<Pdf<'a, W>>{
        output.write_all(b"%PDF-1.7\n%\xB5\xED\xAE\xFB\n")?;
        Ok(
            Pdf {
                output,
                object_offsets: vec![-1, -1, -1],
                page_objects_ids: vec![],
            }
        )
    }

    fn tell(&mut self) -> io::Result<u64> {
        self.output.seek(SeekFrom::Current(0))
    }

    fn render_page<F>(&mut self, width: f32, height: f32, render_contents: F) -> io::Result<()>
        where F: FnOnce(&mut W) -> io::Result<()>,
    {
        let (contents_object_id, content_length) =
            self.write_new_object(move |contents_object_id, pdf| {
                writeln!(pdf.output, "<< /Length {} 0 R", contents_object_id + 1)?;
                writeln!(pdf.output, ">>")?;
                writeln!(pdf.output, "stream")?;

                let start = pdf.tell()?;
                writeln!(pdf.output, "/DeviceRGB cs /DeviceRGB CS")?;
                writeln!(pdf.output, "0.75 0 0 -0.75 0 {} cm", height)?;
                render_contents(pdf.output)?;
                let end = pdf.tell()?;

                writeln!(pdf.output, "endstream")?;
                Ok((contents_object_id, end - start))
            })?;
        self.write_new_object(|length_object_id, pdf| {
            assert_eq!(length_object_id, contents_object_id + 1);
            writeln!(pdf.output, "{}", content_length)

        })?;

        let page_object_id = self.write_new_object(|page_object_id, pdf| {
            writeln!(pdf.output, "<< /Type /Page")?;
            writeln!(pdf.output, "   /Parent {} 0 R", PAGES_OBJECT_ID)?;
            writeln!(pdf.output, "   /Resources << >>")?;
            writeln!(pdf.output, "   /MediaBox [ 0 0 {} {} ]", width, height)?;
            writeln!(pdf.output, "   /Contents {} 0 R", contents_object_id)?;
            writeln!(pdf.output, ">>");
            Ok(page_object_id)
        })?;

        self.page_objects_ids.push(page_object_id);
        Ok(())
    }

    fn write_new_object<F, T>(&mut self, write_content: F) -> io::Result<T>
        where F: FnOnce(usize, &mut Pdf<W>) -> io::Result<T>,
    {
        let id = self.object_offsets.len();
        let offset = self.tell()? as i64;
        self.object_offsets.push(offset);
        self._write_object(id, move |pdf| {
            write_content(id, pdf)
        })
    }

    fn write_object_with_id<F, T>(&mut self, id: usize, write_content: F) -> io::Result<T>
        where F: FnOnce(&mut Pdf<W>) -> io::Result<T>
    {
        assert_eq!(self.object_offsets[id], -1);
        let offset = self.tell()? as i64;
        self.object_offsets[id] = offset;
        self._write_object(id, write_content)
    }

    fn _write_object<F, T>(&mut self, id: usize, write_content: F) -> io::Result<T>
        where F: FnOnce(&mut Pdf<W>) -> io::Result<T>
    {
        writeln!(pdf.output, "{} 0 obj", id)?;
        let result = write_content(self)?;
        writeln!(pdf.output, "endobj")?;
        Ok(result)
    }

    fn finish(&mut self) -> io::Result<()> {
        self._finish()
    }

    fn _finish(&mut self) -> io::Result<()> {
        self.write_object_with_id(PAGES_OBJECT_ID, |pdf| {
            writeln!(pdf.output, "<< /Type /Pages")?;
            writeln!(pdf.output, "   /Count {}", pdf.page_objects_ids.len())?;
            write!(pdf.output, "   /Kids [ ")?;
            for &page_object_id in &pdf.page_objects_ids {
                write!(pdf.output, "{} 0 R ", page_object_id)?;
            }
            writeln!(pdf.output, "]")?;
            writeln!(pdf.output, ">>")?;
            Ok(())
        })?;
        self.write_object_with_id(PAGES_OBJECT_ID, |pdf| {
            writeln!(pdf.output, "<< /Type /Catalog")?;
            writeln!(pdf.output, "   /Pages {} 0 R", PAGES_OBJECT_ID)?;
            writeln!(pdf.output, ">>")?;
            Ok(())
        })?;
        let startxref = self.tell()?;
        writeln!(self.output, "xref")?;
        writeln!(self.output, "0 {}", self.object_offsets.len())?;
        writeln!(self.output, "0000000000 65535 f ")?;
        for &offset in &self.object_offsets[1..] {
            assert!(offset>=0);
            writeln!(self.output, "{:010} 00000 n ", offset)?;
        }
        writeln!(self.output, "trailer")?;
        writeln!(self.output, "<< /Size {}", self.object_offsets.len())?;
        writeln!(self.output, "   /Root {} 0 R", ROOT_OBJECT_ID)?;
        writeln!(self.output, ">>")?;
        writeln!(self.output, "startxref")?;
        writeln!(self.output, "{:?}", startxref)?;
        writeln!(self.output, "%%EOF")?;
        Ok(())
    }
}
