variable "aws_region" {
  type        = string
  description = "AWS region"
}

variable "availability_zones" {
  type        = list(string)
  description = "Availability zones for the subnets"
}

variable "rds_username" {
  type        = string
  description = "RDS username"
}

variable "rds_password" {
  type        = string
  description = "RDS password"
}

variable "ecs_execution_role_arn" {
  type        = string
  description = "Execution role ARN for ECS"
}

variable "app_image" {
  type        = string
  description = "Docker image for the zkp app"
}
