aws_region               = "us-east-1"
availability_zones       = ["us-east-1a", "us-east-1b"]
rds_username             = "production_user"
rds_password             = "production_password"
ecs_execution_role_arn   = "arn:aws:iam::123456789012:role/ecsExecutionRole"
app_image                = "your-ecr-uri/zpk-app:latest"
