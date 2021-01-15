
pub mod  represent{
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair};
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
   
    pub fn cdr<'a> (exp: &'a Exp) -> Result<Exp<'a>, &'static str> {
        match exp {
            Exp::List(_x) => {
                if exp.is_pair() {
                    if let Exp::List(Pair::Cons(_x, y)) = exp { 
                        let z = Exp::List(y);
                        Ok(z )} else {
                            Err("error happens!")
                        }
                } else {Err("not a pair!")}
            }
            _ => Err("type mismatch, not even a List!")
        }
    }
    //pub fn cadr(exp: &Exp) -> Option<&Exp> {Some(exp)}

    pub fn is_tagged_list(exp: &Exp, tag: &'static str) -> bool {
        if exp.is_pair() {
            if let Ok(Exp::Symbol(x)) = car(exp) {
                match x {
                    tag => true,
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair::*};
    use super::represent::*;
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
        // It's painful to build List in Rust...
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

    #[test]
    fn test_cdr() {
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

        assert_eq!(cdr(exp), Ok(Exp::List(t7)));
        assert_eq!(car(&cdr(exp).unwrap()), Ok(v));
        assert_eq!(cdr(car(&cdr(exp).unwrap()).unwrap()), Ok(Exp::List(&s3)));
    }

    #[test]
    fn test_equlity() {
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
       assert_eq!(cdr(&exp).unwrap(), Exp::List(t7));
       assert_eq!(cdr(v), Ok(Exp::List(s3)));
       let ref lrh = Nil;
       let rhs = &Nil;
       assert_eq!(lrh , rhs);
       assert_eq!(cdr(&Exp::List(s1)), Ok(Exp::List(&Nil)));
       
    }

    #[test]
    #[should_panic]
    fn test_inequality() {
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
        assert_eq!(t2, exp);
        assert_eq!(t2, v);
        assert_eq!(t1, s2);
        assert_eq!(t7, t9);
    }

    #[test]
    fn test_tagged_list() {
        let tag = "define";
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
        let tag1 = "define";
        assert_eq!(is_tagged_list(exp, tag1), true);
        assert_eq!(is_tagged_list(t2, "*"), true);
        assert_eq!(is_tagged_list(v, "square"), true);
    }
}

