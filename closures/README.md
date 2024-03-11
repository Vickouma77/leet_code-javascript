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