
use std::ops;

#[derive(Clone, Copy, Debug)]
struct Vector3<T>(T, T, T);

impl<T> Vector3<T> where T: 
    ops::Add<T, Output=T> + ops::Sub<T, Output=T>  + ops::Mul<T, Output=T> + Copy
{
    pub fn norm2(self) -> T { self.0*self.0 + self.1*self.1 + self.2*self.2 }
    pub fn dot(self, v: Vector3<T>) -> T { self.0*v.0 + self.1*v.1 + self.2*v.2 }
    pub fn cross(self, v: Vector3<T>) -> Vector3<T> { Vector3(self.1*v.2-self.2*v.1, self.2*v.0-self.0*v.2, self.0*v.1-self.1*v.0) }
}

impl<T> ops::Add<Vector3<T>> for Vector3<T> where T: ops::Add<T, Output=T>{
    type Output = Vector3<T>;

    fn add(self, v: Vector3<T>) -> Vector3<T> {
        Vector3(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }
}

impl<T> ops::Sub<Vector3<T>> for Vector3<T> where T: ops::Sub<T, Output=T>{
    type Output = Vector3<T>;

    fn sub(self, v: Vector3<T>) -> Vector3<T> {
        Vector3(self.0 - v.0, self.1 - v.1, self.2 - v.2)
    }
}

impl ops::Mul<Vector3<f32>> for f32{
    type Output = Vector3<f32>;

    fn mul(self, v: Vector3<f32>) -> Vector3<f32> {
        Vector3(self * v.0, self * v.1, self * v.2)
    }
}

impl ops::Mul<Vector3<f64>> for f64{
    type Output = Vector3<f64>;

    fn mul(self, v: Vector3<f64>) -> Vector3<f64> {
        Vector3(self * v.0, self * v.1, self * v.2)
    }
}


#[cfg(test)]
mod tests {
    use super::Vector3;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn make() {
        let a = Vector3(1.0,2.0,3.0);
        assert_eq!(a.0, 1.0);
        assert_eq!(a.1, 2.0);
        assert_eq!(a.2, 3.0);
    }

    #[test]
    fn add() {
        let a = Vector3(1.0,2.0,3.0) + Vector3(2.0,3.0,4.0);

        assert_eq!(a.0, 3.0);
        assert_eq!(a.1, 5.0);
        assert_eq!(a.2, 7.0);
    }

    #[test]
    fn sub() {
        let a = Vector3(2.0,3.0,4.0) - Vector3(1.0,2.0,3.0);

        assert_eq!(a.0, 1.0);
        assert_eq!(a.1, 1.0);
        assert_eq!(a.2, 1.0);
    }

    #[test]
    fn mul() {
        let a:Vector3<f32> = 2.0f32 * Vector3(1.0,2.0,3.0);

        assert_eq!(a.0, 2.0);
        assert_eq!(a.1, 4.0);
        assert_eq!(a.2, 6.0);
    }

    #[test]
    fn norm2() {
        let a = Vector3(1.0,2.0,3.0);

        assert_eq!(a.norm2(), 14.0);
    }

    #[test]
    fn dot() {
        let a = Vector3(1.0,2.0,3.0);
        let b = Vector3(2.0,3.0,4.0);

        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross() {
        let a = Vector3(1.0,0.0,0.0);
        let b = Vector3(0.0,2.0,0.0);

        assert_eq!(a.cross(b).0, 0.0);
        assert_eq!(a.cross(b).1, 0.0);
        assert_eq!(a.cross(b).2, 2.0);
    }

}
