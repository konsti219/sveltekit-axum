import { requestFromRep } from "ext:axum_compat/request.js";
import { responseToRep } from "ext:axum_compat/response.js";

globalThis.AxumCompat = { requestFromRep, responseToRep };
