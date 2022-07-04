use std::fmt;

#[derive(Debug)]
struct ComplexNumber(f64, f64);

impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.0, self.1)
    }
}

#[derive(Debug)]
struct ComplexNumberDict {
    real: f64,
    imag: f64,
}

impl fmt::Display for ComplexNumberDict {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let complex_number = ComplexNumber(3.3, 7.2);
    println!("Structure with 2 numbers!");
    println!("{}", complex_number);
    println!("{:?}", complex_number);

    let complex_number_dict = ComplexNumberDict {
        real: 3.3,
        imag: 7.2,
    };
    println!("Structure by dict numbers!");
    println!("{}", complex_number_dict);
    println!("{:?}", complex_number_dict);

    let v = List(vec![1, 2, 3]);
    println!("Vector Structure!");
    println!("{}", v);
    println!("{:?}", v);
}
