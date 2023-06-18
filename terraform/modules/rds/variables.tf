variable "rds_username" {
  type        = string
  description = "Username for the RDS database"
}

variable "rds_password" {
  type        = string
  description = "Password for the RDS database"
}

variable "security_group_id" {
  type        = string
  description = "Security group ID for the RDS database"
}

variable "subnet_ids" {
  type        = list(string)
  description = "Subnet IDs for the RDS database"
}
