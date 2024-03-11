# Javascript Closures

## Introduction to JavaScript closures
In Javascript a closure is a function that references variables in the outer scope from its inner scope. Closures preserve the outer scope inside the inner scope.In Javascript closures are created every time a function is created, at function creation time.To understand closures we first need to understand how `lexical scoping` works.

#### lexical scoping
It defines the scope of a variable by the position of the variable declared in the source code. example:

```
let name = 'sarah';

function greetings() {
    let message = 'Hello';
    console.log(message + ' ' + name);
}
```

* `name` is a global variable and It can be accessed from any where including the `greeting()` function

* `message` is a local variable and is only accessible within the `greeting()` function. Trying to access it outside the function will result to an error.

=> In lexical scoping , scopes can be nested and the inner function can access the variable declared in the outer scope. example:

```
funcion greetings() {
    let message = 'Hello';

    function sayHello() {
        console.log(message);
    }

    sayHello();
}

greetings();
```

* The `greeting()` function creates a local variable named `message` and a function named `sayHello()`.

* The `sayHello()` is the inner function that is available only within the body of the `greeting()` function.

* The `sayHell()` function can access the variables of the outer function such as the `message` variable of the `greeting()` function.

* Inside the `greeting()` function, we call the `sayHello()` function to display the message `Hello`.

#### Closures
```
function greetings() {
    let message = 'Hello';

    function sayHello() {
        console.log(message);
    }

    return sayHello;
}

let hello = greetings();
hello(); // Has access to the message variable
```
* The `greetings()` function returns the `sayHello()` function. The `hello` variable holds the reference to the `sayHello()` function.

* When we call the `hello()` function, it still has access to the `message` variable of the `greetings()` function.

* The `sayHello()` function is a closure because it has access to the `message` variable of the `greetings()` function.

=> NB: Closure is a function that preserves the outer scope in the inner scope. 

> :bulb: practical example of closures is when we create a function that returns another function. The inner function has access to the variables of the outer function. This is a closure. 
```
function counter() {
    let count = 0;

    return function() {
        return ++count;
    }
}

let count = counter();
console.log(count()); // 1
console.log(count()); // 2
console.log(count()); // 3
```
* The `counter()` function returns a function that increments the `count` variable. The `count` variable is a private variable of the `counter()` function.

```
function greetings(message) {
    return function(name) {
        return message + ' ' + name;
    }
}
let hello = greetings('Hello');

console.log(hello('Sarah')); // Hello Sarah
console.log(hello('John')); // Hello John
```
* The `greetings()` function returns a function that takes a `name` parameter and returns the `message` and the `name` concatenated together.

* The `hello` variable holds the reference to the function that takes the `name` parameter.

* When we call the `hello()` function with the `name` parameter, it returns the `message` and the `name` concatenated together.

### closure in a loop
&rarr; consider example below:

```
function createButtons() {
    for (let i = 1; i <= 5; i++) {
        let button = document.createElement('button');
        button.appendChild(document.createTextNode('Button ' + i));
        button.addEventListener('click', function() {
            alert('This is button ' + i);
        });
        document.body.appendChild(button);
    }
}

createButtons();
```
* The `createButtons()` function creates five buttons and adds an event listener to each button.

* When we click on any button, it will display an alert message that says `This is button 6`.

* The reason is that the `i` variable is declared using `let` keyword. The `let` keyword creates a new `i` variable for each iteration of the loop.

* The `i` variable is captured by the closure of the event listener function. When the event listener function is called, it will display the value of the `i` variable which is `6`.

* To fix this issue, we can use an IIFE (Immediately Invoked Function Expression) to create a new scope for each iteration of the loop.

## Summary

* A closure is a function that preserves the outer scope in the inner scope.

* Closures are created every time a function is created, at function creation time.

* A closure is a combination of a function and its ability to remember variables in the outer scope.

* Lexical scoping describes how the JavaScript engine uses the location of the variable in the code to determine where that variable is available.