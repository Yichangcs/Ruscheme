/* a Scheme interpreter implemented by Rust */
// any chain of pairs where the last element has () as it's cdr is a proper list
// in Scheme
#[allow(dead_code)]
pub mod core_of_interpreter {
    #[derive(Debug)]
    pub enum Pair<'a> {
        Cons(Box<&'a Exp<'a>>, Box<&'a Pair<'a>>),
        Nil,
    }

    /* everything is an Exp to be interpreted */
    #[derive(Debug)]
    pub enum Exp<'a> {
        FloatNumber(f32),
        Integer(i32),
        List(&'a Pair<'a>),
        Symbol(&'static str),
        Quote(&'static str),
        SchemeString(&'static str),
    }

    struct Env {}

    struct Procedure {}

    /* core function of the Scheme interpreter */
    fn eval(exp: Exp, env: Env) -> Result<Exp, &'static str> {Ok(exp)}

    fn apply(p: Procedure, args: Exp) -> Result<Exp, &'static str> {Ok(args)}
}

mod  represent{
    use crate::core_of_interpreter::{ Pair, Exp};

    /* operatons on Exp as enum methods */
    #[allow(dead_code)]
    impl<'a> Exp<'a> {
        pub fn is_pair(&self) -> bool { 
            match self {
                &Exp::List(x) => {
                    match x {
                        Pair::Nil => false,
                        _ => true,
                    }
                }
                    _ => false,
            }
        }

        pub fn is_variable(&self) -> bool { 
            self.is_symbol()
        }

        pub fn is_quoted(&self) -> bool { 
            match self {
                Exp::Quote(_x) => true,
                _ => false,   
            }
        }

        pub fn is_string(&self) -> bool {
            match self {
                Exp::SchemeString(_x) => true,
                _ => false,
            }
        }
        
        pub fn is_symbol(&self) -> bool { 
            match self {
                Exp::Symbol(_x) => true,
                _ => false,
            }
        }

        pub fn is_number(&self) -> bool { 
            match self {
                Exp::FloatNumber(_x) => true,
                Exp::Integer(_x) => true,
                _ => false,
            }        
        }

        pub fn is_self_evaluating(&self) -> bool {
            self.is_number() || self.is_string()
        }

        pub fn is_tagged(exp: &Exp, tag: &'static str) -> bool { true }

        pub fn is_assignment(exp: &Exp) -> bool { true }

        pub fn is_definiton(exp: &Exp) -> bool { true }

        pub fn is_lambda(exp: &Exp) -> bool { true }

        pub fn is_if(exp: &Exp) -> bool { true }

        pub fn is_begin(exp: &Exp) -> bool { true }

        pub fn is_application(exp: &Exp) -> bool { true }

        pub fn is_cond(exp: &Exp) -> bool { true }
    }
    /* operations on List variant of Exp */
    pub fn car<'a>(exp: &'a Exp) -> Result<&'a Exp<'a>, &'static str> {
        match exp {
            Exp::List(_x) => {
                if exp.is_pair() {
                     if let Exp::List(Pair::Cons(x, _y)) = exp { 
                        Ok(x) } else {
                            Err("error happens!")
                        }
                } else {Err("not a pair!")}
            }
            _ => Err("type mismatch, not even a List!")
        }
    }
   
    /*
    pub fn cdr<'a, 'b>(exp: &'a Exp, item: &'b Pair) -> Result<&'a Exp, &'static str> {
        match exp {
            Exp::List(_x) => {
                if exp.is_pair() {
                    if let Exp::List(Pair::Cons(_x, y)) = exp { 
                        item = &(*y.to_owned());
                        Ok(&Exp::List(*item)) } else {
                            Err("error happens!")
                        }
                } else {Err("not a pair!")}
            }
            _ => Err("type mismatch, not even a List!")
        }
    }
*/
    //pub fn cadr(exp: &Exp) -> Option<&Exp> {Some(exp)}
}

mod parser {
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    use std::io::BufReader;
    use crate::core_of_interpreter::Exp;

    #[allow(dead_code)]
    pub fn read_scheme_programs_from_stdin(p: &mut Vec<String>) -> io::Result<()> {
        let stdin = io::stdin();
    
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => {
                    if !line.trim().is_empty() {
                        p.push(line);
                    }
                }
                Err(_e) => break,
        }
    }
        Ok(())
    } 

    #[allow(dead_code)]
    pub fn read_scheme_programs_from_file(p: &mut Vec<String>) -> io::Result<()>{
        let f = File::open("scheme.txt")?;
        let reader = BufReader::new(f);
        
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if !line.trim().is_empty() {
                        p.push(line);
                    }
                }
                Err(_e) => break,
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn tokenize(p: &mut Vec<String>) -> Vec<String> {
        let mut ss: Vec<String> = p.into_iter().map(|x| x.replace("(", " ( ")).collect();
        ss = ss.into_iter().map(|x| x.replace(")", " ) ")).collect();
        let mut tokens: Vec<String> = vec![];
        for  item in ss.iter() {
            let mut v = item.trim().split_whitespace().collect::<Vec<_>>().
                                  into_iter().map(|x| x.to_string()).collect(); 
            tokens.append(&mut v);
        }
        tokens
    }

    #[allow(dead_code)]
    pub fn assemble_abstract_syntax_tree(v: &Vec<String>) -> Exp {
        let x: Exp = Exp::FloatNumber(9.0);
        x
    }
}

#[cfg(test)]
mod representing_tests {
    use crate::core_of_interpreter::{Pair::*, Exp};
    use crate::represent::*;

    #[test]
    fn test_is_number() {
        let x = Exp::Integer(3);
        assert_eq!(x.is_number(), true);
    }

    #[test] 
    fn test_is_string() {
        let str = "summer";
        let x = Exp::Symbol(str);
        assert_eq!(x.is_symbol(), true);
    }

    #[test]
    fn test_is_self_evaluating() {
        let x = Exp::FloatNumber(3.14);
        let y = Exp::SchemeString("Winter");
        assert_eq!(x.is_self_evaluating() && y.is_self_evaluating(), true);
    }

    #[test]
    fn test_is_symbol() {
        let x = Exp::Symbol("item");
        assert_eq!(x.is_symbol(), true);
    }

    #[test]
    fn test_is_pair() {
        let a = Box::new(&Exp::Integer(1));
        let b = Box::new(&Exp::Integer(2));
        let c = Box::new(&Exp::Integer(3));
        let d = Box::new(&Nil); 
        let x = &Cons(c, d);
        let y = &Cons(b, Box::new(x));
        let z = &Cons(a, Box::new(y));
        let s = Exp::List(z);
        assert_eq!(s.is_pair(), true);
    }

    #[test]
    fn test_is_quoted() {
        let x = Exp::Quote("'x");
        assert_eq!(x.is_quoted(), true);
    }

    #[test]
    fn test_car() {
        // (define (square x) (* x  x))
        let f1 = Box::new(&Exp::Symbol("define"));
        let y = Box::new(&Exp::Symbol("square"));
        let z = Box::new(&Exp::Symbol("x"));
        let a = Box::new(&Exp::Symbol("*"));
        let b = Box::new(&Exp::Symbol("x"));
        let c = Box::new(&Exp::Symbol("x"));
        let d1 = Box::new(&Nil);
        let d2 = Box::new(&Nil);
        let d3 = Box::new(&Nil);
        // represent (* x x)
        let s1 = &Cons(c, d1);
        let s2 = &Cons(b, Box::new(s1));
        let t1 = &Cons(a, Box::new(s2)); 
        let t2 = &Exp::List(t1);
        let f3 = Box::new(t2);
        // represent (square x)
        let s3 = &Cons(z, d2);
        let t3 = Box::new(s3);
        let t4 = &Cons(y, t3);
        let v = &Exp::List(t4);
        let f2 = Box::new(v);
        // represent (define (square x) (* x x))
        let t5 = &Cons(f3, d3);
        let t6 = Box::new(t5);
        let t7 = &Cons(f2, t6);
        let t8 = Box::new(t7);
        let t9 = &Cons(f1, t8);
        let exp = &Exp::List(t9);
        if let Ok(Exp::Symbol(x)) = car(&exp) {
            assert_eq!(x.to_string(), "define");
        };
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::parser::*;

    #[test]
    fn read_scheme_programs_works() {
        let mut programs: Vec<String> = vec![];
        read_scheme_programs_from_file(&mut programs);
        let mut item = programs.iter();
        assert_eq!(item.next(), Some(&"(define (fac n)".to_string()));
        assert_eq!(item.next(), Some(&"   (if (= n 1)".to_string()));
        assert_eq!(item.next(), Some(&"        1".to_string()));
        assert_eq!(item.next(), Some(&"       (* n".to_string()));
        assert_eq!(item.next(), Some(&"          (fac (- n 1)))))".to_string()));
        assert_eq!(item.next(), None);
    }
    
    #[test]
    fn tokenize_works() {
        let mut programs: Vec<String> = vec![];
        let mut tokens: Vec<String> = vec![];
        read_scheme_programs_from_file(&mut programs);
        tokens = tokenize(&mut programs);
        let s = vec!["(", "define", "(", "fac", "n", ")",
                      "(", "if", "(", "=", "n", "1", ")",
                        "1", "(", "*", "n", "(", "fac",
                          "(", "-", "n",
                             "1", ")", ")", ")", ")", ")"];
        let mut ss: Vec<String> = vec![];
        ss = s.into_iter().map(|x| x.to_string()).collect();
        assert_eq!(ss, tokens);
    }    
}