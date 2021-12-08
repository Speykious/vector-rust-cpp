#ifndef VECTOR3_
#define VECTOR3_

#include <cmath>

class Vector3 {
public:
    Vector3();
    Vector3(const double x, const double y, const double z);
    Vector3(const Vector3 &v);

    // utility operations
    Vector3 &zero();
    Vector3 &set(const double x, const double y, const double z);
    Vector3 &normalize();

    // math operations
    const double norm() const;
    const double sum() const;
    const double dot(const Vector3 &) const;
    const Vector3 cross(const Vector3 &) const;
    const Vector3 abs() const;

    // operators
    Vector3 &operator=(const Vector3 &v); // assignment

    const double operator[](const int i) const; // indexing

    const Vector3 operator-(); // unary negate

    Vector3 &operator+=(const double s); // scalar addition
    Vector3 &operator-=(const double s); // scalar subtraction
    Vector3 &operator*=(const double s); // scalar multiplication
    Vector3 &operator/=(const double s); // scalar division

    Vector3 &operator+=(const Vector3 &v); // vector addition
    Vector3 &operator-=(const Vector3 &v); // vector subtraction
    Vector3 &operator*=(const Vector3 &v); // element-wise multiplication
    Vector3 &operator/=(const Vector3 &v); // element-wise division

    // compare each element with s, return vector of 1
    // or 0 based on test
    const Vector3 operator<(const double s);
    const Vector3 operator>(const double s);

    // element-wise less than comparion, return
    // vector of 1 or 0 based on test
    const Vector3 operator<(const Vector3 &v);

    // element-wise greater than comparion, return
    // vector of 1 or 0 based on test
    const Vector3 operator>(const Vector3 &v);

    bool operator==(const Vector3 &v); // test vector for equality
    bool operator!=(const Vector3 &v); // test vector for inequality

    double x, y, z;
};

#endif // VECTOR3_