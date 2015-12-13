extern crate num;

// public enum NeuronType
//     {
//         UNKNOWN,
//         Relay, //(!:!)
//         Pyramid, //..mMan
//         PyramidPull,
//         Dampner,
//         Function,
//         //Disperser,
//         Periodic,
//         Limit, ///Train
//         LimitwTrain
//     }

trait Network {
//use num::traits::Num;

//pub use log;
    fn new(name: &'static str) -> Self;

    // // Instance methods, only signatures
    // fn name(&self) -> &'static str;
    // fn noise(&self) -> &'static str;
    //
    // // A trait can provide default method definitions
    // fn talk(&self) {
    //     // These definitions can access other methods declared in the same
    //     // trait
    //     println!("{} says {}", self.name(), self.noise());
    // }
    // void LoadInputsIntoModules();
    //  void ProcessFirstBlock();
    //  INeuronBlock GetEntryBlock();
    //  INeuronBlock GetBlock(uint id);
    //  void AddRunningTasks(Task task);
    //  void RemoveRunningTasks(Task task);
}

   // let xs: [i32; 5] = [1, 2, 3, 4, 5];

// public interface INeuronBlock
//    {
//        uint Id { get; set; }
//
//        void LoadVector(byte[] values , uint offset);
//
//        //     //   void SetOutput(byte[] blockInputs);
//        byte[] Process();
//        Task<byte[]> ProcessAsync();
//        Task<byte[]> ProcessAsync(TaskFactory taskFactory);
//        IEnumerable<uint> GetNextBlocks();
//        Connection[] ConnectionsTo { get; }
//    }


// public interface IAnneModule
//    {
//        void LoadInputsIntoModules();
//        void ProcessFirstBlock();
//        INeuronBlock GetEntryBlock();
//        INeuronBlock GetBlock(uint id);
//        void AddRunningTasks(Task task);
//        void RemoveRunningTasks(Task task);
//    }

// struct Dog { name: &'static str }
//
// impl Dog {
//     fn wag_tail(&self) {
//         println!("{} wags tail", self.name);
//     }
// }
//
// // Implement the `Animal` trait for `Dog`
// impl Animal for Dog {
//     // Replace `Self` with the implementor type: `Dog`
//     fn new(name: &'static str) -> Dog {
//         Dog { name: name }
//     }
//
//     fn name(&self) -> &'static str {
//         self.name
//     }
//
//     fn noise(&self) -> &'static str {
//         "woof!"
//     }
//
//     // Default trait methods can be overridden
//     fn talk(&self) {
//         // Traits methods can access the implementor methods
//         self.wag_tail();
//
//         println!("{} says {}", self.name, self.noise());
//     }
// }
