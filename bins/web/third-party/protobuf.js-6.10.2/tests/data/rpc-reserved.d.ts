import * as $protobuf from "../..";
export class MyService extends $protobuf.rpc.Service {
    constructor(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean);
    public static create(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean): MyService;
    public delete(request: IMyRequest, callback: MyService.DeleteCallback): void;
    public delete(request: IMyRequest): Promise<MyResponse>;
}

export namespace MyService {

    type DeleteCallback = (error: (Error|null), response?: MyResponse) => void;
}

export interface IMyRequest {
    path?: (string|null);
}

export class MyRequest implements IMyRequest {
    constructor(properties?: IMyRequest);
    public path: string;
    public static create(properties?: IMyRequest): MyRequest;
    public static encode(message: IMyRequest, writer?: $protobuf.Writer): $protobuf.Writer;
    public static encodeDelimited(message: IMyRequest, writer?: $protobuf.Writer): $protobuf.Writer;
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): MyRequest;
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): MyRequest;
    public static verify(message: { [k: string]: any }): (string|null);
    public static fromObject(object: { [k: string]: any }): MyRequest;
    public static toObject(message: MyRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };
    public toJSON(): { [k: string]: any };
}

export interface IMyResponse {
    status?: (number|null);
}

export class MyResponse implements IMyResponse {
    constructor(properties?: IMyResponse);
    public status: number;
    public static create(properties?: IMyResponse): MyResponse;
    public static encode(message: IMyResponse, writer?: $protobuf.Writer): $protobuf.Writer;
    public static encodeDelimited(message: IMyResponse, writer?: $protobuf.Writer): $protobuf.Writer;
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): MyResponse;
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): MyResponse;
    public static verify(message: { [k: string]: any }): (string|null);
    public static fromObject(object: { [k: string]: any }): MyResponse;
    public static toObject(message: MyResponse, options?: $protobuf.IConversionOptions): { [k: string]: any };
    public toJSON(): { [k: string]: any };
}
