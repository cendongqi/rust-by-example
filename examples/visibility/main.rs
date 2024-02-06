/*
    module里的item默认都是private，除非使用pub声明
 */

mod my_mod {
    // 一个名叫my_mod的模块

    fn private_function() { // module里的item默认都是private
        println!("call `my_mod::private_function)`");
    }

    pub fn function() { // 使用pub声明public
        println!("called `my_mod::function()`");
    }

    pub fn indirect_access() { // 相同模块内的item可以直接访问，即使是private
        println!("called `my_mod::indirect_access()`, that\n>");
        private_function()
    }

    pub mod nested {// 模块可以嵌套
        pub fn function() { // 使用pub声明public
            println!("called `my_mod::nested::function()`");
        }

        fn private_function() { // module里的item默认都是private
            println!("call `my_mod::nested::private_function)`");
        }

        // pub(in path)语法可以指定仅对path内的路径可以访问和当前模块，除此之外都是private
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called `my_mod::nested::public_in_my_mod`, that\n>");
            public_function_in_nested();
        }

        // pub(self)语法仅对当前模块内可见，和默认的效果相同
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // pub(super)仅对其父模块和当前模块可见
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mode()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        println!("called `my_mod::call_public_function_in_my_mod`, that\n>");
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate)仅在当前crate内部所有模块可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate`, that\n>");
    }

    // 嵌套模块遵循一样的规则
    mod private_nested {
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // 即使在子module里声明了更大的可见性，但在private的module里依然受到限制
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // 允许没有歧义的相同名称的item
    function();
    my_mod::function();

    // public item，包括嵌套的modules，可以从父module访问
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate)可以在同个crate任何位置访问
    my_mod::public_function_in_crate();

    // pub(in path)只能在指定路径内调用
    // my_mod::nested::public_function_in_my_mod();

    // private items of a module不可以直接访问，即使用module用pub声明

    // private_function is private
    // my_mod::private_function();

    // private_function is private
    // my_mod::nested::private_function();

    // private_nested是private module
    // my_mod::private_nested::function();
    // my_mod::private_nested::restricted_function();
}
