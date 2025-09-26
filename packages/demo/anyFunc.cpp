#include <iostream>
#include <stdexcept>

// 开始定义我们的泛型函数
template <typename T>
T f(T num1, T num2) {
    return num1 + num2;
}

template <typename K>
void swap_num(K& num1, K& num2) {
    K temp = num1;
    num1 = num2;
    num2 = temp;
}

template <typename T, int size = 10>
class GenericStack {
private:
    // 定义一些属性
    T elements[size];
    int top;  // 栈顶元素

public: 
    GenericStack() : top(-1) {}

    void push(const T& element) {
        if (isFull()) {
            throw std::overflow_error("栈已满，无法入栈");
        }
        elements[++top] = element;
    }

    T pop() {
        if (isEmpty()) {
            throw std::underflow_error("栈为空，无法出栈");
        }
        return elements[top--];
    }

    T peek() const {
        if (isEmpty()) {
            throw std::underflow_error("栈为空，无栈顶元素");
        }
        return elements[top];
    }

    bool isFull() const {
        return top == size - 1;
    }

    bool isEmpty() const {
        return top == -1;
    }

    int get_size() const {
        return top + 1;
    }
};

int main() {
    GenericStack<int> stack;
    stack.push(1);
    stack.push(2);
    stack.push(3);
    std::cout << "栈的大小为: " << stack.get_size() << std::endl;
    std::cout << "栈顶元素为: " << stack.peek() << std::endl;
    std::cout << "出栈元素为: " << stack.pop() << std::endl;
    std::cout << "栈的大小为: " << stack.get_size() << std::endl;
    std::cout << "栈顶元素为: " << stack.peek() << std::endl;
}