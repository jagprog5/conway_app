use std::fmt::Display;
use rand::{thread_rng, Rng};
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde::de::{Deserialize, Deserializer, Visitor, Error};

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<const W: usize, const H: usize> {
    cells: [[bool; W]; H],
}

pub struct GridIter<const W: usize, const H: usize> {
    grid: Grid<W, H>,
}

impl<const W: usize, const H: usize> Grid<W, H> {
    pub fn empty() -> Self {
        Self { cells: [[false; W]; H] }
    }
    
    pub fn random() -> Self {
        let mut cells = [[false; W]; H];
        for row in cells.iter_mut() {
            thread_rng().fill(&mut row[..]);
        }
        Self { cells }
    }

    // get value at coordinate, false if out of bound
    fn at(&self, y: usize, x: usize) -> bool {
        *self
            .cells
            .get(y)
            .map(|row| row.get(x).unwrap_or(&false))
            .unwrap_or(&false)
    }

    fn count_active<I>(&self, coords: I) -> usize
    where
        I: Iterator<Item = (usize, usize)>
    {
        let mut ret = 0usize;
        for (y, x) in coords {
            if self.at(y, x) {
                ret += 1;
            }
        }
        ret
    }

    fn count_neighbors(&self, y: usize, x: usize) -> usize {
        fn id(n: usize) -> Option<usize> {
            Some(n)
        }

        fn inc(n: usize) -> Option<usize> {
            n.checked_add(1)
        }

        fn dec(n: usize) -> Option<usize> {
            n.checked_sub(1)
        }

        let neighbors = [
            (inc(y), inc(x)),
            (dec(y), dec(x)),
            (inc(y), dec(x)),
            (dec(y), inc(x)),
            (id(y), inc(x)),
            (inc(y), id(x)),
            (id(y), dec(x)),
            (dec(y), id(x)),
        ];

        let valid_neighbors = neighbors.iter().filter_map(|(y, x)| {
            let y = (*y)?;
            let x = (*x)?;
            Some((y, x))
        });

        self.count_active(valid_neighbors)
    }

    fn next_at(&self, y: usize, x: usize) -> bool {
        matches!(
            (self.at(y, x), self.count_neighbors(y, x)),
            (true, 2) | (true, 3) | (false, 3)
        )
    }
}

impl<const W: usize, const H: usize> Display for Grid<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/")?;
        for _ in 0..W {
            write!(f, "=")?;
        }
        writeln!(f, "\\")?;
        for i in 0..H {
            write!(f, "|")?;
            for j in 0..W {
                write!(
                    f,
                    "{}",
                    match self.cells[i][j] {
                        true => 'X',
                        false => ' ',
                    }
                )?;
            }
            writeln!(f, "|")?;
        }
        write!(f, "\\")?;
        for _ in 0..W {
            write!(f, "=")?;
        }
        write!(f, "/")?;
        Ok(())
    }
}

impl<const W: usize, const H: usize> IntoIterator for Grid<W, H> {
    type Item = Grid<W, H>;

    type IntoIter = GridIter<W, H>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter { grid: self }
    }
}

impl<const W: usize, const H: usize> Iterator for GridIter<W, H> {
    type Item = Grid<W, H>;

    fn next(&mut self) -> Option<Self::Item> {
        let cells = [[false; W]; H];
        let mut new_grid = Self::Item { cells };
        for y in 0..H {
            for x in 0..W {
                new_grid.cells[y][x] = self.grid.next_at(y, x);
            }
        }
        self.grid = new_grid.clone();
        Some(new_grid)
    }
}

// serde does not support const generics yet
// there are packages that help but they don't work for 2d arrays.
// doing this by hand

impl<const W: usize, const H: usize> Serialize for Grid<W, H> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(W * H))?;
        for y in 0..H {
            for x in 0..W {
                seq.serialize_element(&self.cells[y][x])?;
            }
        }
        seq.end()
    }
}

struct GridDeserializer<const W: usize, const H: usize>;

impl<'de, const W: usize, const H: usize> Visitor<'de> for GridDeserializer<W, H> {
    type Value = Grid<W, H>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("sequence of bool")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>, {
        let mut ret = Grid::empty();
        let mut y = 0usize;
        let mut x = 0usize;

        while let Some(val) = seq.next_element()? {
            if y == H {
                return Err(A::Error::custom(format!("too many elements specified")));
            }
            ret.cells[y][x] = val;
            x += 1;
            if x == W {
                x = 0;
                y += 1;
            }
        }
        if x != 0 || y != H {
            return Err(A::Error::custom(format!("not enough elements specified")))
        }
        Ok(ret)
    }
}

impl<'de, const W: usize, const H: usize> Deserialize<'de> for Grid<W, H> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = GridDeserializer::<W, H>;
        return deserializer.deserialize_seq(visitor).map_err(|err| {
            err
        });
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        *place = Deserialize::deserialize(deserializer)?;
        Ok(())
    }
}
