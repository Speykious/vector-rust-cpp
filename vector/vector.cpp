#include "vector.hpp"
#include <cmath>
#include <string>

Vector3::Vector3() {
    x = 0.0;
    y = 0.0;
    z = 0.0;
}

Vector3::Vector3(double x, double y, double z) {
    this->x = x;
    this->y = x;
    this->z = x;
}

double Vector3::norm() {
    return std::sqrt(x * x + y * y + z * z);
}

Vector3 Vector3::normalized() {
    double n = norm();
    return *this / n;
}

Vector3 Vector3::operator+(const Vector3 other) const {
    return Vector3(
        x + other.x,
        y + other.y,
        z + other.z
    );
}

Vector3 Vector3::operator-(const Vector3 other) const {
    return Vector3(
        x - other.x,
        y - other.y,
        z - other.z
    );
}

Vector3 Vector3::operator*(const double factor) const {
    return Vector3(
        x * factor,
        y * factor,
        z * factor
    );
}

Vector3 Vector3::operator/(const double factor) const {
    return Vector3(
        x / factor,
        y / factor,
        z / factor
    );
}

std::string Vector3::toString() {
    return "("
        + std::to_string(x) + ", "
        + std::to_string(y) + ", "
        + std::to_string(z) + ")";
}