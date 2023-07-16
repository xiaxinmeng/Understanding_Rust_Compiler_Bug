rust
// Traits and base impls:
trait Archive {
    type Archived;
}

type Archived<T> = <T as Archive>::Archived;

trait Deserialize<T: Archive, D: ?Sized> {}

impl<T, const N: usize> Archive for [T; N]
where
    T: Archive,
{
    type Archived = [T::Archived; N];
}

impl<T: Archive, D: ?Sized, const N: usize> Deserialize<[T; N], D> for [T::Archived; N]
where
    T: Archive,
    T::Archived: Deserialize<T, D>,
{
}

// A happy struct + impls that works perfectly fine:
pub enum Happy<T> {
    A([T; 10]),
    B([T; 10]),
}

impl<T> Archive for Happy<T>
where
    T: Archive,
    [T; 10]: Archive,
    [T; 10]: Archive,
{
    type Archived = Happy<Archived<T>>;
}

impl<D: ?Sized, T> Deserialize<Happy<T>, D> for Archived<Happy<T>>
where
    T: Archive,
    [T; 10]: Archive,
    Archived<[T; 10]>: Deserialize<[T; 10], D>,
    [T; 10]: Archive,
    Archived<[T; 10]>: Deserialize<[T; 10], D>,
{
}

// A sad struct + impls that causes a compiler error:
pub const MYSIZE: usize = 10;

pub enum Sad<T> {
    A([T; MYSIZE]),
    B([T; MYSIZE]),
}

impl<T> Archive for Sad<T>
where
    T: Archive,
    [T; MYSIZE]: Archive,
    [T; MYSIZE]: Archive,
{
    type Archived = Sad<Archived<T>>;
}

impl<D: ?Sized, T> Deserialize<Sad<T>, D> for Archived<Sad<T>>
where
    T: Archive,
    [T; MYSIZE]: Archive,
    Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>,
    [T; MYSIZE]: Archive,                               // 💣
    Archived<[T; MYSIZE]>: Deserialize<[T; MYSIZE], D>, // 💣
{
}
