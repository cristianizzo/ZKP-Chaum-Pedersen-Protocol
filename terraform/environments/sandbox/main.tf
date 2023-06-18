provider "aws" {
  region = var.aws_region
}

module "network" {
  source             = "../../modules/network"
  vpc_cidr           = "10.0.0.0/16"
  subnet_cidrs       = ["10.0.1.0/24", "10.0.2.0/24"]
  availability_zones = var.availability_zones
  environment        = "development"
}

module "rds" {
  source               = "../../modules/rds"
  vpc_id               = module.network.vpc_id
  subnet_ids           = module.network.subnet_ids
  security_group_id = module.network.rds_security_group_id
  rds_instance_class   = "db.t3.medium"
  rds_allocated_storage = 20
  rds_username         = var.rds_username
  rds_password         = var.rds_password
}

module "ecs" {
  source                   = "../../modules/ecs"
  vpc_id                   = module.network.vpc_id
  subnet_ids               = module.network.subnet_ids
  security_group_id = module.network.ecs_security_group_id
  ecs_execution_role_arn   = var.ecs_execution_role_arn
  app_image                = var.app_image,
  database_uri              = module.rds.endpoint
  subnets                  = module.network.subnet_ids
}
