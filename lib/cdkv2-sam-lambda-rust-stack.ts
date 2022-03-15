import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as logs from 'aws-cdk-lib/aws-logs';

export class Cdkv2SamLambdaRustStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    // Function that calls Rust
    new lambda.Function(this, 'rust-lambda', {
      description:
        'Rust function on Lambda using the custom runtime',
      code: lambda.Code.fromAsset(
        'functions/target/x86_64-unknown-linux-musl/release'
      ),
      runtime: lambda.Runtime.PROVIDED_AL2,
      architecture: lambda.Architecture.X86_64,
      handler: 'not.required',
      environment: {
        RUST_BACKTRACE: '1',
      },
      logRetention: logs.RetentionDays.ONE_WEEK,
    })
  }
}
