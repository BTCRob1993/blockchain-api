locals {
  env_settings = {
    "staging" : {
      autoscaling : {
        desired_count = 1,
        min_capacity  = 1,
        max_capacity  = 2,
      }
    },
    "prod" : {
      autoscaling : {
        desired_count = 1,
        min_capacity  = 2,
        max_capacity  = 5,
      }
    }
  }
}

data "aws_s3_bucket" "geoip" {
  bucket = data.terraform_remote_state.infra_aws.outputs.geoip_bucked_id
}

resource "aws_prometheus_workspace" "prometheus" {
  alias = "prometheus-${module.this.id}"
}

# ECS Cluster, Task, Service, and Load Balancer for our app
module "ecs" {
  source  = "./ecs"
  context = module.this

  # Cluster
  ecr_repository_url        = local.ecr_repository_url
  image_version             = var.image_version
  task_cpu                  = 512
  task_memory               = 1024
  autoscaling_desired_count = local.env_settings[local.stage].autoscaling.desired_count
  autoscaling_min_capacity  = local.env_settings[local.stage].autoscaling.min_capacity
  autoscaling_max_capacity  = local.env_settings[local.stage].autoscaling.max_capacity

  # DNS
  route53_zones              = local.zones
  route53_zones_certificates = local.zones_certificates

  # Network
  vpc_id                          = module.vpc.vpc_id
  public_subnets                  = module.vpc.public_subnets
  private_subnets                 = module.vpc.private_subnets
  database_subnets                = module.vpc.database_subnets
  allowed_app_ingress_cidr_blocks = module.vpc.vpc_cidr_block
  allowed_lb_ingress_cidr_blocks  = module.vpc.vpc_cidr_block

  # Application
  port                          = 8080
  project_cache_endpoint_read   = module.redis.endpoint
  project_cache_endpoint_write  = module.redis.endpoint
  identity_cache_endpoint_read  = module.redis.endpoint
  identity_cache_endpoint_write = module.redis.endpoint

  # Providers
  infura_project_id = var.infura_project_id
  pokt_project_id   = var.pokt_project_id
  zerion_api_key    = var.zerion_api_key

  # Project Registry
  registry_api_endpoint   = var.registry_api_endpoint
  registry_api_auth_token = var.registry_api_auth_token
  project_cache_ttl       = var.project_cache_ttl

  # Analytics
  analytics_datalake_bucket_name = data.terraform_remote_state.datalake.outputs.datalake_bucket_id
  analytics_datalake_kms_key_arn = data.terraform_remote_state.datalake.outputs.datalake_kms_key_arn

  # Monitoring
  prometheus_workspace_id = aws_prometheus_workspace.prometheus.id
  prometheus_endpoint     = aws_prometheus_workspace.prometheus.prometheus_endpoint

  # GeoIP
  geoip_db_bucket_name = data.aws_s3_bucket.geoip.id
  geoip_db_key         = var.geoip_db_key
}
