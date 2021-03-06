use crate::{api, commands::Args, db::DB, schema::tags, Error};

use diesel::prelude::*;

/// Remove a key value pair from the tags.  
pub fn delete(args: Args) -> Result<(), Error> {
    let conn = DB.get()?;
    let key = args
        .params
        .get("key")
        .ok_or("Unable to retrieve param: key")?;

    match diesel::delete(tags::table.filter(tags::key.eq(key))).execute(&conn) {
        Ok(_) => args.msg.react(args.cx, "✅")?,
        Err(_) => api::send_reply(&args, "A database error occurred when deleting the tag.")?,
    }
    Ok(())
}

/// Add a key value pair to the tags.  
pub fn post(args: Args) -> Result<(), Error> {
    let conn = DB.get()?;

    let key = args
        .params
        .get("key")
        .ok_or("Unable to retrieve param: key")?;

    let value = args
        .params
        .get("value")
        .ok_or("Unable to retrieve param: value")?;

    match diesel::insert_into(tags::table)
        .values((tags::key.eq(key), tags::value.eq(value)))
        .execute(&conn)
    {
        Ok(_) => args.msg.react(args.cx, "✅")?,
        Err(_) => api::send_reply(&args, "A database error occurred when creating the tag.")?,
    }
    Ok(())
}

/// Update an existing tag.
pub fn update(args: Args) -> Result<(), Error> {
    let conn = DB.get()?;

    let key = args
        .params
        .get("key")
        .ok_or("Unable to retrieve param: key")?;

    let value = args
        .params
        .get("value")
        .ok_or("Unable to retrieve param: value")?;

    match diesel::update(tags::table.filter(tags::key.eq(key)))
        .set(tags::value.eq(value))
        .execute(&conn)
    {
        Ok(_) => args.msg.react(args.cx, "✅")?,
        Err(_) => api::send_reply(&args, "A database error occurred when updating the tag.")?,
    }

    Ok(())
}

/// Retrieve a value by key from the tags.  
pub fn get(args: Args) -> Result<(), Error> {
    let conn = DB.get()?;

    let key = args.params.get("key").ok_or("unable to read params")?;

    let results = tags::table
        .filter(tags::key.eq(key))
        .load::<(i32, String, String)>(&conn)?;

    if results.is_empty() {
        api::send_reply(&args, &format!("Tag not found for `{}`", key))?;
    } else {
        api::send_reply(&args, &results[0].2)?;
    }

    Ok(())
}

/// Retrieve all tags
pub fn get_all(args: Args) -> Result<(), Error> {
    let conn = DB.get()?;

    let results = tags::table.load::<(i32, String, String)>(&conn)?;

    if results.is_empty() {
        api::send_reply(&args, "No tags found")?;
    } else {
        let tags = &results.iter().fold(String::new(), |prev, row| {
            if prev.len() < 1980 {
                prev + &row.1 + "\n"
            } else {
                prev
            }
        });

        api::send_reply(&args, &format!("All tags: ```\n{}```", &tags))?;
    }

    Ok(())
}

/// Print the help message
pub fn help(args: Args) -> Result<(), Error> {
    let help_string = "```
?tags create {key} value...     Create a tag.  Limited to WG & Teams.
?tags update {key} value...     Update a tag.  Limited to WG & Teams.
?tags delete {key}              Delete a tag.  Limited to WG & Teams.
?tags help                      This menu.
?tags                           Get all the tags.
?tag {key}                      Get a specific tag.
```";
    api::send_reply(&args, &help_string)?;
    Ok(())
}
