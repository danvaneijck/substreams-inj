// @generated
pub mod sf {
    pub mod cosmos {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.cosmos.type.v2)
            pub mod v2 {
                include!("sf.cosmos.type.v2.rs");
                // @@protoc_insertion_point(sf.cosmos.type.v2)
            }
        }
    }
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod cosmos {
            // @@protoc_insertion_point(attribute:sf.substreams.cosmos.v1)
            pub mod v1 {
                include!("sf.substreams.cosmos.v1.rs");
                // @@protoc_insertion_point(sf.substreams.cosmos.v1)
            }
        }
        pub mod rpc {
            // @@protoc_insertion_point(attribute:sf.substreams.rpc.v2)
            pub mod v2 {
                include!("sf.substreams.rpc.v2.rs");
                // @@protoc_insertion_point(sf.substreams.rpc.v2)
            }
        }
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
}
pub mod trades {
    // @@protoc_insertion_point(attribute:trades.v1)
    pub mod v1 {
        include!("trades.v1.rs");
        // @@protoc_insertion_point(trades.v1)
    }
}
