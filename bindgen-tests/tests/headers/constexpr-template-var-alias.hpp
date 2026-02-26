// bindgen-flags: -- -std=c++14

// Conditional template
template<bool B, typename T, typename U>
struct conditional { typedef T type; };

template<typename T, typename U>
struct conditional<false, T, U> { typedef U type; };

template<bool B, typename T, typename U>
using conditional_t = typename conditional<B, T, U>::type;

// Constexpr template variable - bindgen cannot resolve this as a template argument
template <class Alloc>
constexpr bool is_simple_alloc = true;

// A template struct
template <class _Val_types>
struct Container {
    _Val_types data;
};

// A type alias whose argument involves a constexpr template variable.
// instantiate_template returns None for is_simple_alloc<Alloc>, so the alias
// falls back to the bare Container template definition. Without the fix this
// leaks _Val_types into the generated binding.
template <class Alloc>
class vector {
public:
    using Scary_val = Container<conditional_t<is_simple_alloc<Alloc>, int, char>>;

    Scary_val field;
};
