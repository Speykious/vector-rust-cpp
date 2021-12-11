#ifndef __VECTOR3__
#define __VECTOR3__

#include <string>

class Vector3 {
public:
    double x, y, z;

    Vector3();
    Vector3(double x, double y, double z);

    double norm();
    Vector3 normalized();

    Vector3 operator+(const Vector3 other) const;
    Vector3 operator-(const Vector3 other) const;
    Vector3 operator*(const double factor) const;
    Vector3 operator/(const double factor) const;
    
    std::string toString();
};

#endif