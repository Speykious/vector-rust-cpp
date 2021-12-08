#ifndef __VECTOR3__
#define __VECTOR3__

#include <string>

class Vector3 {
    double x, y, z;

public:
    Vector3();
    Vector3(double x, double y, double z);

    double norm();
    Vector3 normalized();

    Vector3 operator+(const Vector3 other);
    Vector3 operator-(const Vector3 other);
    Vector3 operator*(const double factor);
    Vector3 operator/(const double factor);
    
    std::string toString();
};

#endif