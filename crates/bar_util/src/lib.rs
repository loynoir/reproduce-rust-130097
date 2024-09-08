pub use bar::Bar;

mod bar {
    pub struct Bar<T>(T);

    impl Bar<i32> {
        /// # Safety
        ///
        /// be careful
        pub const unsafe fn from_unchecked(value: i32) -> Self {
            Bar(value)
        }
    }
}
