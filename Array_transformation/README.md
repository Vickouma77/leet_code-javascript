# Array Transformation

## Table of Contents

1. JavaScript Array Length
2. JavaScript Array Map
3. JavaScript Array Filter
4. JavaScript Array Reduce

## JavaScript Array Length

`length` property of an array is an unsigned, 32-bit integer that is always numerically greater than the highest index in the array.The value of the length is `2^32`. It means that an array can hold up to `4294967296 (2^32)` elements.
The `length` property behaves differently depending on the array types including dense and sparse.

#### Dense Array
A dense array is an array in which the elements are contiguous. The `length` property of a dense array is always one more than the highest index of the array. > :bulb: `length = highest index + 1` (indexes starting at zero.)
The `length` property is used to get the number of elements in the array. For example:

```javascript
let arr = [1, 2, 3, 4, 5];
console.log(arr.length); // 5
```

#### Sparse Array
A sparse array is an array in which the elements are not contiguous. The `length` property of a sparse array is always greater than the highest index of the array. The `length` property is used to get the number of elements in the array. For example:

```javascript
let arr = [10, 20, 30];
arr[10] = 100;
console.log(arr.length); // 11
```