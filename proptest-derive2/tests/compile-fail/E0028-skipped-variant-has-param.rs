#[macro_use]
extern crate proptest_derive;

#[derive(Debug, Arbitrary)] //~ ERROR: 2 errors 
                            //~| [proptest_derive, E0028]
                            //~| [proptest_derive, E0006]
enum NonFatal {
    #[proptest(skip, no_params)]
    V1,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T0 {
    #[proptest(skip, no_params)]
    V1,
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T1 {
    #[proptest(skip, no_params)]
    V1(u8),
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T2 {
    #[proptest(skip, no_params)]
    V1 {
        field: String,
    },
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T3 {
    #[proptest(skip, params = "u8")]
    V1,
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T4 {
    #[proptest(skip, params = "u8")]
    V1(u8),
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T5 {
    #[proptest(skip, params = "u8")]
    V1 {
        field: String,
    },
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T6 {
    #[proptest(skip)]
    V1(
        #[proptest(params = "u8")]
        u8
    ),
    V2,
}

#[derive(Debug, Arbitrary)] //~ ERROR: [proptest_derive, E0028]
enum T7 {
    #[proptest(skip)]
    V1 {
        #[proptest(params = "u8")]
        field: String,
    },
    V2,
}
