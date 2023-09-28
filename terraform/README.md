# Terraform Infrastructure

Get yourself some AWS creds and then init your workspace:

`terraform -chdir=terraform init`

Use the dev workspace:

`terraform -chdir=terraform workspace select dev`

Now you can apply the changes:

`terraform -chdir=terraform apply -var-file="vars/$(terraform -chdir=terraform workspace show).tfvars"`

## Deploying local code changes

```bash
nano .env # set AWS access keys and GRAFANA_AUTH
source .env
./terraform/deploy-dev.sh
```

### macOS considerations

If you get this error:

```
assertion failed [find_leftmost_allocation_holding_lock(interval) == nullptr]: interval being added overlaps existing allocation
(VMAllocationTracker.cpp:322 add)
```

Try disabling "Use Rosetta for x86/amd64 emulation on Apple Silicon" in Docker Desktop settings.

#### Remote building

If amd64 builds are too slow on your Mac (likely), consider using a remote builder on a linux/amd64 host:

```bash
docker buildx create --name=remote-amd64 --driver=docker-container ssh://<my-amd64-host>
BUILD_ARGS="--builder=remote-amd64 --load" ./terraform/deploy-dev.sh
```
<!-- BEGIN_TF_DOCS -->

## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | >= 1.0 |
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | >= 5.7 |
| <a name="requirement_grafana"></a> [grafana](#requirement\_grafana) | >= 2.1 |
| <a name="requirement_random"></a> [random](#requirement\_random) | 3.5.1 |
## Providers

| Name | Version |
|------|---------|
| <a name="provider_aws"></a> [aws](#provider\_aws) | 5.18.1 |
| <a name="provider_random"></a> [random](#provider\_random) | 3.5.1 |
| <a name="provider_terraform"></a> [terraform](#provider\_terraform) | n/a |
## Modules

| Name | Source | Version |
|------|--------|---------|
| <a name="module_cloudwatch"></a> [cloudwatch](#module\_cloudwatch) | ./cloudwatch | n/a |
| <a name="module_dns_certificate"></a> [dns\_certificate](#module\_dns\_certificate) | app.terraform.io/wallet-connect/dns/aws | 0.1.3 |
| <a name="module_ecs"></a> [ecs](#module\_ecs) | ./ecs | n/a |
| <a name="module_monitoring"></a> [monitoring](#module\_monitoring) | ./monitoring | n/a |
| <a name="module_redis"></a> [redis](#module\_redis) | ./redis | n/a |
| <a name="module_this"></a> [this](#module\_this) | app.terraform.io/wallet-connect/label/null | 0.3.2 |
| <a name="module_vpc"></a> [vpc](#module\_vpc) | terraform-aws-modules/vpc/aws | ~> 5.0 |
| <a name="module_vpc_endpoints"></a> [vpc\_endpoints](#module\_vpc\_endpoints) | terraform-aws-modules/vpc/aws//modules/vpc-endpoints | 5.1 |
| <a name="module_vpc_flow_s3_bucket"></a> [vpc\_flow\_s3\_bucket](#module\_vpc\_flow\_s3\_bucket) | terraform-aws-modules/s3-bucket/aws | ~> 3.14 |

## Inputs
| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_betterstack_cloudwatch_webhook"></a> [betterstack\_cloudwatch\_webhook](#input\_betterstack\_cloudwatch\_webhook) | The BetterStack webhook to send CloudWatch alerts to |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_geoip_db_key"></a> [geoip\_db\_key](#input\_geoip\_db\_key) | The name to the GeoIP database |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_grafana_auth"></a> [grafana\_auth](#input\_grafana\_auth) | The API Token for the Grafana instance |  <pre lang="json">string</pre> |  <pre lang="json">""</pre> |  no |
| <a name="input_image_version"></a> [image\_version](#input\_image\_version) | The version of the image to deploy |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_infura_project_id"></a> [infura\_project\_id](#input\_infura\_project\_id) | The project ID for Infura |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_notification_channels"></a> [notification\_channels](#input\_notification\_channels) | The notification channels to send alerts to |  <pre lang="json">list(any)</pre> |  <pre lang="json">[]</pre> |  no |
| <a name="input_pokt_project_id"></a> [pokt\_project\_id](#input\_pokt\_project\_id) | The project ID for POKT |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_project_cache_ttl"></a> [project\_cache\_ttl](#input\_project\_cache\_ttl) | The TTL for project data cache |  <pre lang="json">number</pre> |  <pre lang="json">300</pre> |  no |
| <a name="input_registry_api_auth_token"></a> [registry\_api\_auth\_token](#input\_registry\_api\_auth\_token) | The auth token for the registry API |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_registry_api_endpoint"></a> [registry\_api\_endpoint](#input\_registry\_api\_endpoint) | The endpoint of the registry API |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_zerion_api_key"></a> [zerion\_api\_key](#input\_zerion\_api\_key) | The API key for Zerion |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
## Outputs

No outputs.


<!-- END_TF_DOCS -->
