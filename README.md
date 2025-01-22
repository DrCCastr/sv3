# Sun Language Interpreter

> **Note:** This project is no longer actively maintained. The Sun Language Interpreter was developed as an educational tool and is currently in a basic and incomplete state. Contributions are welcome but are not guaranteed to be reviewed or merged.

---

## 🌟 **Features**

- **Core Components**:
  - **AST-Based Evaluation**: The interpreter processes an Abstract Syntax Tree (AST) to execute Sun Language code.
  - **Dynamic Typing**: The language supports dynamic variables through `SunVariable`.
  - **Basic Data Types**:
    - Numbers
    - Objects
    - Nil (null-like)
  - **Expressions**:
    - Binary operations (e.g., addition, subtraction).
    - Assignments and variable declarations.
  - **Program Execution**: Executes a root `Program` node containing a sequence of statements.

- **Error Handling**:
  - If a node is not recognized, the interpreter provides detailed error messages and halts execution.

---

## 🔧 **How It Works**

### **Evaluation**
The interpreter processes nodes of the AST recursively, matching each node type:

- **`NumericLiteral`**: Returns a numeric value.
- **`BinaryExpr`**: Evaluates binary expressions (e.g., `+`, `-`, `*`, `/`).
- **`Identifier`**: Resolves variables in the current environment.
- **`ObjectLiteral`**: Creates objects.
- **`VarDeclarationStmt`**: Declares variables in the environment.
- **`AssignmentExpr`**: Assigns values to variables.
- **`Program`**: Executes the entire program.

If a node type is unimplemented or unsupported, an error is thrown.

---

## 📄 **Code Example**

Below is a conceptual example of the Sun Language syntax and how it would be processed:

### **Sun Language Code**:
```sun
var x = 10;
var y = 20;
var z = x + y;
```

### **Evaluation Flow**:
1. **`VarDeclarationStmt`**: Declares `x`, `y`, and `z`.
2. **`AssignmentExpr`**: Assigns `10` to `x`, `20` to `y`.
3. **`BinaryExpr`**: Evaluates `x + y` and assigns the result to `z`.

---

## 🚀 **Getting Started**

### **Requirements**
- Rust 1.70 or higher.
- Basic knowledge of how interpreters work.

### **Setup**
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/sun-language-interpreter.git
   cd sun-language-interpreter
   ```
2. Build the project:
   ```bash
   cargo build
   ```
3. Run the interpreter with an example AST:
   ```bash
   cargo run
   ```

---

## 📚 **Interpreter Design**

The interpreter consists of several key modules:

- **`ast`**: Defines the structure of nodes in the AST.
- **`runtime::value`**: Handles data types like numbers and objects.
- **`environmment`**: Manages variable scopes and environments.
- **`eval`**: Implements the evaluation logic for different node types.

---

## 🔮 **Future Enhancements**

- Add support for control flow (e.g., `if` statements, loops).
- Implement more data types, such as strings and arrays.
- Improve error reporting with more descriptive messages.
- Add REPL support for interactive programming.

---

## 📝 **License**

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details. 

---

## 🤝 **Contributing**

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

---

Enjoy exploring the Sun Language and learning more about interpreters! 🌞
