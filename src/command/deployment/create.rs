#![allow(clippy::enum_variant_names)]

use anyhow::Result;
use clap::Args;
use graphql_client::{GraphQLQuery, Response};

use crate::{
    api::ApiClient,
    command::deployment::create::create_deployment::{
        DeploymentService, DeploymentTier, KatanaConfigInput, ServiceConfigInput, ToriiConfigInput,
        Variables,
    },
};

use self::create_deployment::ServiceInput;

use super::configs::CreateCommands;

type Long = u64;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/command/deployment/create.graphql",
    response_derives = "Debug"
)]
pub struct CreateDeployment;

#[derive(clap::ValueEnum, Clone, Debug, serde::Serialize)]
pub enum Tier {
    Basic,
}

#[derive(Debug, Args, serde::Serialize)]
#[command(next_help_heading = "Create deployment options")]
pub struct CreateOptions {
    #[arg(short, long = "name")]
    #[arg(help = "The name of the project.")]
    pub name: String,
    #[arg(short, long, default_value = "basic")]
    #[arg(value_name = "tier")]
    #[arg(help = "Deployment tier.")]
    pub tier: Tier,
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Create options")]
pub struct CreateArgs {
    #[command(flatten)]
    options: CreateOptions,

    #[command(subcommand)]
    create_commands: CreateCommands,
}

impl CreateArgs {
    pub async fn run(&self) -> Result<()> {
        let service = match &self.create_commands {
            CreateCommands::Katana(config) => ServiceInput {
                type_: DeploymentService::katana,
                version: None,
                config: Some(ServiceConfigInput {
                    katana: Some(KatanaConfigInput {
                        block_time: config.block_time,
                        fork_rpc_url: config.fork_rpc_url.clone(),
                        fork_block_number: config.fork_block_number,
                        seed: match &config.seed {
                            Some(seed) => seed.clone(),
                            None => rand::random::<u64>().to_string(),
                        },
                        total_accounts: config.total_accounts,
                    }),
                    torii: None,
                }),
            },
            CreateCommands::Torii(config) => ServiceInput {
                type_: DeploymentService::torii,
                version: None,
                config: Some(ServiceConfigInput {
                    katana: None,
                    torii: Some(ToriiConfigInput {
                        rpc: config.rpc.clone(),
                        world: format!("{:#x}", config.world),
                        start_block: Some(config.start_block),
                    }),
                }),
            },
        };

        let tier = match &self.options.tier {
            Tier::Basic => DeploymentTier::basic,
        };

        let request_body = CreateDeployment::build_query(Variables {
            name: self.options.name.clone(),
            tier,
            service,
            wait: Some(true),
        });

        let client = ApiClient::new();
        let res: Response<create_deployment::ResponseData> = client.post(&request_body).await?;
        if let Some(errors) = res.errors.clone() {
            for err in errors {
                println!("Error: {}", err.message);
            }
        }

        println!("{:#?}", res.data);
        Ok(())
    }
}
