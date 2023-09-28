resource "random_pet" "this" {
  length = 2
}

locals {
  ecr_repository_url = data.terraform_remote_state.org.outputs.accounts.wl.blockchain[local.stage].ecr-url

  stage = lookup({
    "blockchain-staging"    = "staging",
    "blockchain-prod"       = "prod",
    "blockchain-wl-staging" = "staging",
    "blockchain-wl-prod"    = "prod",
    "wl-staging"            = "staging",
    "wl-prod"               = "prod",
    "staging"               = "staging",
    "prod"                  = "prod",
  }, terraform.workspace, terraform.workspace)
}
