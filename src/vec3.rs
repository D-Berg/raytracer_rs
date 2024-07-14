use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};



pub type Point3 = Vec3;

#[derive(Debug)]
pub struct Vec3 {
    e: [f64; 3]
}


impl Vec3 {

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {

        Vec3 {
            e: [e0, e1, e2]
        }
    }
    
    pub fn zeros() -> Self {

        Vec3 {
            e: [0.0; 3]
        }

    }

    pub fn ones() -> Self {

        Vec3 {
            e: [1.0; 3]
        }

    }

    fn dot(&self, other: &Vec3) -> f64 {
        
        let mut res: f64 = 0.0;

        for i in 0..3 {

            res += self[i] * other[i];


        }

        return res;
    }

    fn cross(&self, other: &Vec3) -> Vec3 {

        let mut res = Vec3::zeros();

        res.e[0] = self.e[1] * other.e[2] - self.e[2] * other.e[1];
        res.e[1] = self.e[2] * other.e[0] - self.e[0] * other.e[2];
        res.e[1] = self.e[0] * other.e[1] - self.e[1] * other.e[0];

        return res;

    }

    fn len(&self) -> Vec3 {

        todo!()

    }



    pub fn x(&self) -> f64 {

        return self[0]

    }

    pub fn y(&self) -> f64 {

        return self[1]

    }

    pub fn z(&self) -> f64 {

        return self[2]

    }
}

// Operator overloading for Vec3
//Note that (for any binary operator @):
//
// - &A @ &A produces a new Array
// - B @ A consumes B, updates it with the result, and returns it
// - B @ &A consumes B, updates it with the result, and returns it
// - C @= &A performs an arithmetic operation in place
//
// - &A @ K or K @ &A which produces a new Array
// - B @ K or K @ B which consumes B, updates it with the result and returns it
// - C @= K which performs an arithmetic operation in place

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        &mut self.e[i]
    }
}

/// &V + f64 Produces new Vec3
impl Add<f64> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {

        let mut res = Vec3::zeros();

        for i in 0..3 {

            res[i] = self[i] + rhs;

        }
        
        return res;
        
    }

}
 
/// &V1 + &V2 produces new Vec3
impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        
        let mut res = Vec3::zeros();


        for i in 0..3 {

            res[i] = self[i] + rhs[i];

        }

        return res;

    }

}

/// V1 + &V2 which consumes V1, updates it and returns it.
impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: &Vec3) -> Self::Output {

        for i in 0..3 {
            self[i] += rhs[i];
        }

        return self;
    }
}

/// &V1 + V2 which consumes V2, updates it and returns it.
impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, mut rhs: Vec3) -> Self::Output {

        for i in 0..3 {
            rhs[i] += self[i];
        }

        return rhs;
    }
}



// SUB Vec scalar ---------------
impl Sub<f64> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {

        let mut res = Vec3::zeros();

        for i in 0..3 {

            res[i] = self[i] - rhs;

        }
        
        return res;
        
    }

}


// B - A consumes B, updates it with the result, and returns it
impl Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {

        for i in 0..3 {

            self[i] -= rhs[i];

        }
        
        self
    }
    
}

// B @ &A which consumes B, updates it with the result, and returns it
impl Sub<&Vec3> for Vec3{
    type Output = Self;

    fn sub(mut self, rhs: &Vec3) -> Self::Output {
        
        for i in 0..3 {
            self[i] -= rhs[i];
        }

        self

    }
    
}

/// &A @ &A produces a new Array
impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        
        let mut res = Vec3::zeros();


        for i in 0..3 {

            res[i] = self[i] - rhs[i];

        }

        return res;

    }

}

// MUL -----------


/// &A * K which produces a new vec3
impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {

        let mut res = Vec3::zeros();

        for i in 0..3 {

            res[i] = self[i] * rhs;

        }
        
        return res;
        
    }

}

/// K * &A which produces a new array
impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    
    fn mul(self, rhs: &Vec3) -> Self::Output {

        let mut res = Vec3::zeros();

        for i in 0..3 {

            res[i] = self * rhs[i];

        }


        return res;
        
    }
}

/// A * K which consumes A and updates it with the result
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {

        for i in 0..3 {

            self[i] *= rhs;

        }

        return self;
        
    }
    
}

/// K * V which consumes V and updates it with the result
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, mut rhs: Vec3) -> Self::Output {

        for i in 0..3 {

            rhs[i] *= self;

        }

        return rhs;
        
    }
    
}


impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        
        let mut res = Vec3::zeros();


        for i in 0..3 {

            res[i] = self[i] * rhs[i];

        }

        return res;

    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {

        let mut res = Vec3::zeros();

        for i in 0..3 {

            res[i] = self[i] / rhs;

        }
        
        return res;
        
    }

}

impl Div for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        
        let mut res = Vec3::zeros();


        for i in 0..3 {

            res[i] = self[i] / rhs[i];

        }

        return res;

    }
}



// Unit tests

#[test]
fn vec_add_f64() {

    let v1 = Vec3::ones();


    let v2 = &v1 + 10.0;

    let answer = Vec3::new(11.0, 11.0, 11.0);

    
    
    for i in 0..3 {
        assert_eq!(v2[i], answer[i]);
    }



}

#[test]
fn vec_add_vec() {

    let v1 = Vec3::ones();
    let v2 = Vec3::ones();

    let answer = Vec3::new(2.0, 2.0, 2.0);

    let v3 = &v1 + &v2;

    for i in 0..3 {

        assert_eq!(v3[i], answer[i]);

    }

}

#[test]
fn vec_sub_f64() {

    let v1 = Vec3::ones();


    let v2 = &v1 - 10.0;

    let answer = Vec3::new(-9.0, -9.0, -9.0);

    
    
    for i in 0..3 {
        assert_eq!(v2[i], answer[i]);
    }



}
#[test]
fn vec_sub_vec() {

    let v1 = Vec3::ones();
    let v2 = Vec3::new(2.0, 2.0, 2.0);

    let answer = Vec3::ones();

    let v3 = &v2 - &v1;

    for i in 0..3 {

        assert_eq!(v3[i], answer[i]);

    }

}

#[test]
fn vec_mul_vec() {

    let v1 = Vec3::ones();
    let v2 = Vec3::ones();

    let answer = Vec3::new(1.0, 1.0, 1.0);

    let v3 = &v1 * &v2;

    for i in 0..3 {

        assert_eq!(v3[i], answer[i]);

    }


}

#[test]
fn vec_div_vec() {

    let v1 = Vec3::new(2.0, 2.0, 2.0);
    let v2 = Vec3::new(12.0, 10.0, 20.0);

    let answer = Vec3::new(6.0, 5.0, 10.0);

    let v3 = &v2 / &v1;

    for i in 0..3 {

        assert_eq!(v3[i], answer[i]);

    }


}


// TODO Implement more tests for operations
