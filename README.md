# lemma

## Deployment

`lemma` uses [Cargo Lambda](https://www.cargo-lambda.info/) for builds and AWS SAM for deployment.

```bash
# Build to stage environment
./deploy/deploy-aws.sh -p [AWS_PROFILE]

# Build to production environment
./deploy/deploy-aws.sh -p [AWS_PROFILE] --stage prod
```
