#include "vector.hpp"

Vector3::Vector3() {
  x = 0.0;
  y = 0.0;
  z = 0.0;
}

Vector3::Vector3(const Vector3 &v) {
  x = v.x;
  y = v.y;
  z = v.z;
}

Vector3::Vector3(const double x, const double y, const double z) {
  this->x = x;
  this->y = y;
  this->z = z;
}

Vector3 &Vector3::zero() {
  x = 0.0;
  y = 0.0;
  z = 0.0;
  return *this;
}

Vector3 &Vector3::set(const double x, const double y, const double z) {
  this->x = x;
  this->y = y;
  this->z = z;
  return *this;
}

inline const double Vector3::operator[](const int i) const {
    switch (i) {
        case 0: return x;
        case 1: return y;
        case 2: return z;
        default: return 0.0;
    }
}

inline const Vector3 Vector3::abs() const {
  return Vector3(std::abs(x), std::abs(y), std::abs(z));
}

inline const double Vector3::sum() const { return x + y + z; }

inline const double Vector3::dot(const Vector3 &v) const {
  return x * v[0] + y * v[1] + z * v[2];
}

inline const Vector3 Vector3::cross(const Vector3 &v) const {
  return Vector3((y * v[2]) - (z * v[1]), (z * v[0]) - (x * v[2]),
                 (x * v[1]) - (y * v[0]));
}

inline const double Vector3::norm() const {
  return (double)sqrt(dot(*this)); // cast to type
}

Vector3 &Vector3::normalize() {
  double n = norm();
  if (n) {
    x /= n;
    y /= n;
    z /= n;
  }
  return *this;
}

Vector3 &Vector3::operator=(const Vector3 &v) {
  x = v[0];
  y = v[1];
  z = v[2];
  return *this;
}

Vector3 &Vector3::operator+=(const Vector3 &v) {
  x += v[0];
  y += v[1];
  z += v[2];
  return *this;
}

Vector3 &Vector3::operator+=(double v) {
  x += v;
  y += v;
  z += v;
  return *this;
}

Vector3 &Vector3::operator-=(const Vector3 &v) {
  x -= v[0];
  y -= v[1];
  z -= v[2];
  return *this;
}

Vector3 &Vector3::operator-=(double v) {
  x -= v;
  y -= v;
  z -= v;
  return *this;
}

Vector3 &Vector3::operator*=(double v) {
  x *= v;
  y *= v;
  z *= v;
  return *this;
}

Vector3 &Vector3::operator*=(const Vector3 &v) {
  x *= v[0];
  y *= v[1];
  z *= v[2];
  return *this;
}

Vector3 &Vector3::operator/=(double v) {
  x /= v;
  y /= v;
  z /= v;
  return *this;
}

Vector3 &Vector3::operator/=(const Vector3 &v) {
  x /= v[0];
  y /= v[1];
  z /= v[2];
  return *this;
}

inline const Vector3 Vector3::operator<(const double s) {
  return Vector3(x < s, y < s, z < s);
}

inline const Vector3 Vector3::operator>(const double s) {
  return Vector3(x > s, y > s, z > s);
}

inline const Vector3 Vector3::operator<(const Vector3 &v) {
  return Vector3(x < v[0], y < v[1], z < v[2]);
}

inline const Vector3 Vector3::operator>(const Vector3 &v) {
  return Vector3(x > v[0], y > v[1], z > v[2]);
}

inline const Vector3 Vector3::operator-() { return Vector3(-x, -y, -z); }

inline bool Vector3::operator==(const Vector3 &v) {
  return x == v[0] && y == v[1] && z == v[2];
}

inline bool Vector3::operator!=(const Vector3 &v) {
  return x != v[0] || y != v[1] || z != v[2];
}

inline const Vector3 operator&&(const Vector3 &v1, const Vector3 &v2) {
  return Vector3(v1[0] && v2[0], v1[1] && v2[1], v1[2] && v2[2]);
}

inline const Vector3 operator||(const Vector3 &v1, const Vector3 &v2) {
  return Vector3(v1[0] || v2[0], v1[1] || v2[1], v1[2] || v2[2]);
}

inline const Vector3 operator+(const Vector3 &v, const double &s) {
  return Vector3(v) += s;
}

inline const Vector3 operator-(const Vector3 &v, const double &s) {
  return Vector3(v) -= s;
}

inline const Vector3 operator*(const Vector3 &v, const double &s) {
  return Vector3(v) *= s;
}

inline const Vector3 operator/(const Vector3 &v, const double &s) {
  return Vector3(v) /= s;
}

inline const Vector3 operator+(const Vector3 &v1, const Vector3 &v2) {
  return Vector3(v1) += v2;
}

inline const Vector3 operator-(const Vector3 &v1, const Vector3 &v2) {
  return Vector3(v1) -= v2;
}

inline const double operator*(const Vector3 &v1, const Vector3 &v2) {
  return v1.dot(v2);
}

inline const Vector3 operator^(const Vector3 &v1, const Vector3 &v2) {
  return v1.cross(v2);
}