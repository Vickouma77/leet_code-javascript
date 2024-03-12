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

#### Setting the length property
The `length` property can be set to truncate an array at any time. If the new `length` is less than the current `length`, the array will be truncated to the new `length`. If the new `length` is greater than the current `length`, the array will be extended to the new `length` and the new elements will be `undefined`. For example:

```javascript
let arr = [1, 2, 3, 4, 5];
arr.length = 3;
console.log(arr); // [1, 2, 3]

arr.length = 5;
console.log(arr); // [1, 2, 3, undefined, undefined]
```

## JavaScript Array Map

The `map()` method creates a new array with the results of calling a provided function on every element in the calling array. The `map()` method does not change the original array. For example:

```javascript
let arr = [1, 2, 3, 4, 5];
let newArr = arr.map((element) => element * 2);
console.log(newArr); // [2, 4, 6, 8, 10]
```

The following illustrates `map()` method with an array of objects:

```javascript
let arr = [
  { id: 1, name: 'John' },
  { id: 2, name: 'Jane' },
  { id: 3, name: 'Doe' }
];
let newArr = arr.map((element) => element.name);
console.log(newArr); // ['John', 'Jane', 'Doe']
```

`map()` explanation:
```javascript
arrayObject.map(callback[,contextObject]);
```
The `map()` method calls a callback function on every element of an array and returns a new array that contains the results.
The `map()` method takes two named arguments, the first one is required whereas the second one is optional.
Similar to the other iterative method such as  `every()`,  `some()`,  `filter()`, `forEach()` and  `sort()`, the `callback()` function has the following form &rarr;
```javascript
function callback(currentElement,index,array){
  // ... 
}
```
The `callback()` function takes three arguments:
* `currentElement`: The current element being processed in the array.
* `index`: The index of the current element being processed in the array.
* `array`: The array that `map()` was called upon.

```
>:bulb: The `map()` method does not change the original array.
```

## JavaScript Array Filter