use deadpool_postgres::{Pool, ManagerConfig, RecyclingMethod, Runtime, PoolError};
use dotenvy::dotenv;
use tokio_postgres::NoTls;
use tonic::{transport::Server, Request, Response, Status};
use api::{
    parser_integration_service_server::{ParserIntegrationService, ParserIntegrationServiceServer},
    ParserQueryRequest, ParsedContentResponse,
};

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Debug)]
struct ParserIntegrationServer{
    pool: Pool
}


struct KeywordAndProductId{
    keyword_text: String,
}

impl KeywordAndProductId {
    fn from_row(row: tokio_postgres::Row) -> Result<Self, PoolError> {
        Ok(KeywordAndProductId{
            keyword_text: row.get("keyword_text"),
        })
    }
}

fn handle_db_error(e: tokio_postgres::Error) -> Status {
    Status::internal(format!("Database error: {}", e))
}

fn handle_pool_error(e: PoolError) -> Status {
    Status::internal(format!("Pool error: {}", e))
}

#[tonic::async_trait]
impl ParserIntegrationService  for ParserIntegrationServer  {
    async fn get_parsed_content(
        &self,
        request: Request<ParserQueryRequest>,
    ) -> Result<Response<ParsedContentResponse>, Status> {
        let input = request.into_inner().query_id;
        let client = &self.pool.get().await.map_err(handle_pool_error)?;
        let keyword_and_product_id_query = client.query("SELECT keywords.keyword_text FROM public.keyword_products join keywords on keywords.keyword_id = keyword_products.keyword_id  where keyword_products.product_id = $1 limit 30;", &[&input]).await.map_err(handle_db_error)?;
        let mut words = Vec::new();
        for query in keyword_and_product_id_query{
            let answer = KeywordAndProductId::from_row(query).map_err(handle_pool_error)?;
            words.push(answer.keyword_text);
        }

        Ok(Response::new(ParsedContentResponse {
            parsed_terms: words
        }))
    }
}

#[derive(Debug, serde::Deserialize)]
struct Config {
    pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    use std::env;
    let out_dir = env::var("PG__HOST").unwrap();    
    println!("{}", out_dir);
    let pool_size: u32 = 20;
    let cfg = Config::from_env()?;

    let mgr = deadpool_postgres::Manager::new(cfg.pg.get_pg_config().expect("Not find env file"), NoTls);
        
    ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let pool = Pool::builder(mgr)
        .max_size(pool_size as usize)
        .runtime(Runtime::Tokio1)
        .build()
        .unwrap();
    let addr = format!("0.0.0.0:{}", dotenvy::var("PORT")?).parse()?;
    let processor = ParserIntegrationServer{
        pool
    };
    Server::builder()
        .add_service(ParserIntegrationServiceServer::new(processor))
        .serve(addr)
        .await?;
    Ok(())
  
}
