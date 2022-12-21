use tealr::{
    rlu::{rlua::Result, TealData, TealDataMethods, UserData},
    TypeName, TypeWalker,
};
//this example shows how to use the create_named_parameters! macro to create methods which has names for their parameters in the documentation
#[derive(Clone, UserData, TypeName)]
struct Example {}

impl TealData for Example {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        //this creates a new struct that will contain our parameters
        //it has a field with the name `field_1` of type `String`
        //and a field with the name `field_2` of type `i64`
        tealr::rlu::create_named_parameters!(
            TestName with
                field_1 : String,
                field_2 : i64,
        );
        methods.add_method("example_method", |_, _, a: TestName| {
            println!("field_1 = {}; field_2 = {}", a.field_1, a.field_2);
            Ok(())
        });
    }
}

fn main() -> Result<()> {
    //lets first generate the definition file
    let file_contents = TypeWalker::new() //creates the generator
        .process_type::<Example>()
        .generate_global("test")
        .expect("oh no :(");

    //normally you would now save the file somewhere.
    println!("{}\n ", file_contents);

    //lua is still using position parameters as normal.
    tealr::rlu::rlua::Lua::new().context(|ctx| {
        let globals = ctx.globals();
        globals.set("test", Example {})?;
        let code = "test:example_method(\"field_1 is a string\", 3)";
        ctx.load(code).set_name("test?")?.eval()?;
        Ok(())
    })?;
    Ok(())
}