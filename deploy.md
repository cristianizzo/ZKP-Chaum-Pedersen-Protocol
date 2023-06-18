# Rust ZKP Application Deployment with Terraform

This README explains how to deploy a Rust application that uses gRPC and a PostgreSQL database using Terraform on AWS. The application makes use of Zero-Knowledge Proofs (ZKP) as described in "Cryptography: An Introduction (3rd Edition) Nigel Smart" page 377, section "3. Sigma Protocols", subsection "3.2. Chaum–Pedersen Protocol".

## Prerequisites

- [AWS Account](https://aws.amazon.com/)
- [Terraform](https://www.terraform.io/downloads.html) (>= 0.12)
- [Docker](https://www.docker.com/products/docker-desktop)
- AWS CLI configured with appropriate permissions

## Steps for Deployment

1. **Set AWS Credentials**: Make sure your AWS credentials are set either via environment variables or AWS CLI configuration.

2. **Configure Environment Variables**: Edit the `.env` file to include any environment variables required for your application, such as database credentials.

3. **Push Docker Image to AWS ECR**: Create a repository in AWS ECR and push your image there. Replace <account_id> with your AWS account id.
   ```
    # Authenticate Docker with ECR
    aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin <account_id>.dkr.ecr.us-east-1.amazonaws.com

    # Tag the image
    docker tag zkp-app:latest <account_id>.dkr.ecr.us-east-1.amazonaws.com/zkp-app:latest

    # Push the image to ECR
    docker push <account_id>.dkr.ecr.us-east-1.amazonaws.com/zkp-app:latest
   ```

4. **Initialize Terraform**: Navigate to the `terraform/` directory and run:
   ```
   terraform init
   ```

5. **Apply Terraform Configuration**: Still in the `terraform/` directory, run:
   ```
   terraform apply
   ```
   This command will prompt you to confirm that you want to create the resources defined in the Terraform files. Type `yes` to proceed.

6. **Monitor the Deployment**: Once the `terraform apply` command completes, your resources should be deploying to AWS. You can monitor the status in the AWS Console.

7. **Access the Application**: Once deployment is complete, you can access your application by navigating to the public IP address or domain of your EC2 instance or load balancer.

## Clean Up Resources

To avoid incurring unnecessary costs, make sure to destroy the resources created once you are done testing or using them:

```
terraform destroy
```

This command will prompt you to confirm that you want to destroy the resources. Type `yes` to proceed.

## Security Note

Remember to follow security best practices when deploying applications, especially in production. This includes restricting network access, using encryption, and regularly rotating credentials.
