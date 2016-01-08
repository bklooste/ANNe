extern crate anne;

pub mod tst_module;
pub mod tst_mut_block;

#[test]
fn it_works() {
    assert_eq!(4, anne::add_two(2));
}


// full compile tests
// [Theory, MemberData("GetModules")]
//      public void test_serialize(ModuleDesign value)
//      {
//          var str = new Compiler().Serialize( value);
//           Assert.True(string.IsNullOrEmpty(str) == false);
//      }
//
//      [Theory, MemberData("GetModules")]
//      public void test_deserialize(ModuleDesign value)
//      {
//          var str = new Compiler().Serialize(value);
//          var result = new Compiler().DeSerialize(str);
//          Assert.True(result != null);
//      }
//
//
//      public static IEnumerable<object[]> GetModules()
//      {
//
//          var testData = new CreateTestModule();
//
//          return testData.GetAllTestModules();
//
//
//
//
//      }
