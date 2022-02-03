# rust-exercise-1

#### 1. What is the differences between pointer type: &, &mut, Box, Rc, Arc, *const, *mut

```
&: immutable reference
 - doesn't take ownership of the data
 - lifetime is checked by the compiler
 - borrow can happen indefinitely in the same scope
 - exclusive with &mut

&mut: mutable reference
 - doesn't take ownership of the data
 - lifetime is checked by the compiler
 - can be borrowed only once in the same scope
 - exclusive with &

Box: unique pointer
 - take ownership of the data (and move the data from stack to heap)
 - single ownership
 - free the memory automatically after dropped

Rc: referenced-counted pointer
 - take ownership of the data (and move the data from stack to heap)
 - multiple ownership (count ownership)
 - immutable borrowed (if not using inner mutability)
 - free the memory when the reference counting reaches 0
 - can cause memory leak in reference cycle (consider using weak)

Arc: atomic RC
 - same as RC except that it's atomic
 - expensive than RC

*const: immutable raw pointer
 - unsafe to dereference
 - lifetime and ownership is not working
 - cannot be reassigned
 - nullable


*mut: mutable raw pointer
 - unsafe to dereference
 - lifetime and ownership is not working
 - can be reassigned
 - nullable
```

#### 2. Rust slice คืออะไร มาใช้ manipulate อะไรได้บ้าง
 - `str`, `[T]`
 - refer to part of the contiguous collection (array string)
 - size not known at compile time; can only exist in a pointer (`&`, `Box`)
 - `&[T]` is a preferable choice for function parameter to `&Vec<T>` because it allow to pass a part of vector to function (more generic)

#### 3.  trait คืออะไร ใช้งานตอนไหน มีผลกับการออกแบบ architecture ยังไง
- trait is a feature that allow abstraction in written functions
- you can write a function that doesn't need to accept every. It just need to implement certain methods. (You don't need to write functions for every types that implement the methods)
- allow other programmer to write code to "plug-in" to your method that accept trait object (and vice-versa)

#### 4. Iterator คืออะไร ใช้ยังไง เกี่ยวพันกับ Rust standard collection ยังไง
- iterator is a trait(s) that allow us to sequentially draw or borrow data from any type in a systematically, seamless way
- work by providing `next` method to see the next data
- `for i in val` desugars to `for i in IntoIterator::into_iter(val)`
- `for i in &val` desugars to `for i in val.iter()`
- `for i in &mut val` desugars to `for i in val.iter_mut()`
- Rust standard collections implement iterator traits to allow syntax `for i in collection`

#### 5. Rust standard collection มีอะไรบ้าง มีอะไรที่เหมือนกันและต่างกันบ้าง

##### Vec, VecDeque, LinkedList
 - ordered sequence
 - Vec offer O(1) append performance, O(n) insert performance, O(1) random access performance
 - LinkedList offer O(1) prepend performance, O(n) insert performance, O(n) random access performance
 - VecDeque is essentially Vec with also O(1) prepend performance.
 - VecDeque requires a little bit extra space compared to Vec
 - Vec and VecDeque worst-case performance of insert, append, prepend are O(n) since it may need to re-allocate new array in the heap.

##### HashMap, BTreeMap
 - mapped collection
 - key is unique
 - BTreeMap's keys are sorted. Therefore, BTreeMap requires that key is sortable
 - General performance of HashMap is O(1) while the BTreeMap is O(log(n))

##### HashSet, BTreeSet
 - essentially map without value (has only key)
 - usage: stores unique values

##### BinaryHeap
 - priority queue
