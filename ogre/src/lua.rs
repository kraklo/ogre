use mlua::Lua;

pub struct CardScript {
    filepath: String,
}

impl CardScript {
    pub fn get_lua_state() -> Lua {
        let lua = Lua::new();
        let globals = lua.globals();

        lua
    }
}
