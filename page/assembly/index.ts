import { storage, context } from "near-sdk-as";
import { PageData, body, content, meta, skeleton } from "./profile";
import { Web4Request, Web4Response, htmlResponse } from "./web4";
import { status } from "./web4";
import { bodyUrl } from "./web4";

const WEB4_IPFS = "web4:ipfs";
const WEB4_OWNER_ID = "web4:owner";
const PROFILE_DATA = "data";

function assertOwner(): void {
  // NOTE: Can change this check to alow different owners
  assert(context.sender == get_owner(), "Only owner can call this method");
}

export function init(data: PageData, ipfs: string, owner_id: string): void {
  const initialized: bool = storage.getPrimitive<bool>("init", false);
  assert(!initialized, "Already initialized");
  storage.set<bool>("init", true);

  storage.set(WEB4_IPFS, ipfs);
  storage.set(WEB4_OWNER_ID, owner_id);
  storage.set(PROFILE_DATA, data);
}

export function get_owner(): string {
  return (storage.getString(WEB4_OWNER_ID) || context.contractName)!;
}

export function get_data(): PageData | null {
  return storage.get<PageData>(PROFILE_DATA);
}

export function set_data(data: PageData): void {
  assertOwner();

  storage.set(PROFILE_DATA, data);
}

export function web4_get(request: Web4Request): Web4Response {
  if (request.path == "/") {
    const data = storage.getSome<PageData>(PROFILE_DATA);

    const pageContent = content(data);
    const pageMeta = meta(data.title, data.description);

    const pageBody = body(pageContent);

    const pageHtml = skeleton(pageMeta, pageBody);

    return htmlResponse(pageHtml);
  }

  if (
    request.path.endsWith(".css") ||
    request.path.endsWith(".png") ||
    request.path.endsWith(".svg")
  ) {
    const ipfs = storage.getString(WEB4_IPFS)!;
    return bodyUrl(`${ipfs}${request.path}`);
  }

  // by default return Not Found
  return status(404);
}
