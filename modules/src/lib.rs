mod outer {
    pub fn public_function() {
        println!("This is a public function");
        private_function(); // Can access private function
    }
    
    fn private_function() {
        println!("This is a private function");
    }

    //call innner private function from outer module
    //pub fn call_inner_private_function() {
       // inner::inner_private_function();
    //}
    
    pub mod inner {
        pub fn inner_public_function() {
            println!("This is a public function in an inner module");
            // Can access parent's private function
            super::private_function();
        }
        
        fn inner_private_function() {
            println!("This is a private function in an inner module");
        }

        pub fn call_outer_private_function() {
            // Can access private function in the same module
            super::private_function();
        }
    }
}

fn main() {
    // Can access public function
    outer::public_function();
    
    // Can access public function in public inner module
    outer::inner::inner_public_function();

    outer::inner::call_outer_private_function();
    
    // These would fail - private functions
    // outer::private_function();
    // outer::inner::inner_private_function();
}