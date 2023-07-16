rust
fn main() {
    let writer = WriteImpl;
    let mut encoder = EncoderImpl::<_>::new(writer);
    Item.encode(&mut encoder);
}

pub trait Encode {
    fn encode<E: Encoder>(&self, encoder: E);
}

pub trait Encoder {
    type W: Writer;

    fn writer(&mut self) -> &mut Self::W;
}

pub trait Writer {
    fn write(&mut self);
}

struct Item;

impl Encode for Item {
    fn encode<E: Encoder>(&self, mut encoder: E) {
        encoder.writer().write();
        self.encode(&mut encoder);
    }
}

struct EncoderImpl<W: Writer> {
    writer: W,
}

impl<W: Writer> EncoderImpl<W> {
    pub fn new(writer: W) -> EncoderImpl<W> {
        EncoderImpl { writer }
    }
}

impl<W: Writer> Encoder for EncoderImpl<W> {
    type W = W;

    fn writer(&mut self) -> &mut Self::W {
        &mut self.writer
    }
}

struct WriteImpl;

impl Writer for WriteImpl {
    fn write(&mut self) {}
}

impl<'a, T> Encoder for &'a mut T
where
    T: Encoder,
{
    type W = T::W;

    fn writer(&mut self) -> &mut Self::W {
        T::writer(self)
    }
}
