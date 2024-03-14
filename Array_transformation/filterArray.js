/**
 * @param {number[]} arr
 * @param {Function} fn
 * @return {number[]}
 */
var filter = function(arr, fn) {
    let filteredArr = [];
    for(let i = 0; i < arr.length; i++) {
        if (fn(arr[i], i)) {
            filteredArr.push(arr[i])
        }
    }
    return filteredArr
};

/**
 * using forEach
 */

var filter = function(arr, fn) {
    let filteredArr = [];
    arr.forEach((item, index) => {
        if (fn(item, index)) {
            filteredArr.push(item)
        }
    });
    return filteredArr
}