// Re-export the generated client and connect helper
import { connectSquelService, type SquelServiceClient } from "./generated/squel-service";
export { connectSquelService };
export type { SquelServiceClient };

// Connection state
let client: SquelServiceClient | null = null;

export async function connect(url: string = "ws://127.0.0.1:9000"): Promise<SquelServiceClient> {
  if (client) {
    return client;
  }
  client = await connectSquelService(url);
  return client;
}

export function getClient(): SquelServiceClient | null {
  return client;
}

export function disconnect(): void {
  client = null;
}
