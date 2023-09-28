module "cloudwatch" {
  source  = "./cloudwatch"
  context = module.this.context

  webhook_url = var.betterstack_cloudwatch_webhook

  ecs_cluster_name = module.ecs.ecs_cluster_name
  ecs_service_name = module.ecs.ecs_service_name

  redis_cluster_id = module.redis.cluster_id
}
