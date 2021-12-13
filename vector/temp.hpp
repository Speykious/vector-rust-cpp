#ifndef __TEMP__
#define __TEMP__

template<typename T>
class Temp {
public:
    T some_value;
    
    Temp();
};

template<>
inline Temp<double>::Temp() {
    some_value = 1.5;
}

template<>
inline Temp<int>::Temp() {
    some_value = 3;
}

#endif