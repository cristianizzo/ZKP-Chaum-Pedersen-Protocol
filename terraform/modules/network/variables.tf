variable "vpc_cidr" {
  type        = string
  description = "CIDR block for the VPC"
}

variable "subnet_cidrs" {
  type        = list(string)
  description = "List of CIDR blocks for the subnets"
}

variable "availability_zones" {
  type        = list(string)
  description = "List of availability zones for the subnets"
}

variable "environment" {
  type        = string
  description = "Environment (e.g. 'production', 'sandbox')"
}
