use postgres::{Client, NoTls};

pub(crate) struct PostgresSequenceOut{
    client: postgres::Client
}
impl PostgresSequenceOut{
    pub(crate) fn new(database_uri: &str, genome_names: &Vec<&str>) -> Result<PostgresSequenceOut, Box<dyn std::error::Error + 'static>>{
        let mut client = Client::connect(database_uri, NoTls)?;
        let mut create_table_query = String::from("CREATE TABLE IF NOT EXISTS sequences(
            sequence TEXT PRIMARY KEY, ");
        for current_name in genome_names{
            create_table_query.push_str(current_name);
            create_table_query.push_str(" INT,")
        }
        create_table_query.pop();
        create_table_query.push_str(");");
        client.batch_execute(&*create_table_query)?;
        Ok(PostgresSequenceOut{
            client,
        })
    }
}

impl PostgresSequenceOut{
    pub(crate) fn add_answer(&mut self, current_sequence: String, repetition_number: usize, other_genome_name: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let mut create_table_query = format!("INSERT INTO sequences (sequence, {}) VALUES
        ('{}', {}) ON CONFLICT (sequence) DO UPDATE SET {} = {};", other_genome_name, current_sequence,
                                             repetition_number, other_genome_name, repetition_number);
        self.client.batch_execute(&*create_table_query)?;
        Ok(())
    }
}
