
struct GuestCount(i32);

impl Default for GuestCount {
    fn default() -> Self {
        Self(2)
    }
}

impl From<i32> for GuestCount {
    fn from(value: i32) -> Self {
        GuestCount(value)
    }
}

#[derive(Default)]
struct Madness {
    iter_count: i32,
    guest_count: GuestCount
}

struct MapIt<Iter, Func, Item> where Iter: Iterator, Func: FnMut(Iter::Item) -> Item
{
    iter: Iter,
    f: Func
}

impl<Item, Iter, Func> Iterator for MapIt<Iter,Func, Item> where Iter: Iterator, Func: FnMut(Iter::Item) -> Item {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next();
        x.map(|v| (self.f)(v))
    }
}

impl Madness {
    fn mappit<Func>(self, f: Func) -> MapIt<Self, Func, i32>  where Func: FnMut(i32)-> i32  {
        MapIt {iter: self, f}
    }
}



impl Iterator for Madness {
    type Item = i32;
fn next(&mut self) -> Option<Self::Item> {
    self.iter_count += 1;
    if(self.iter_count >= 5){ return None };
    Some(4)
    }
}


pub(crate) fn main(){

    let mad = Madness::default();

    let selfmappedmad = mad.mappit(|x| x + 20);

    selfmappedmad.for_each(|val|{
        println!("{val}")
    });
}